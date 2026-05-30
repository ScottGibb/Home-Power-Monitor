use std::time::Duration;

use post_haste::agent::{Agent, Inbox};

use crate::{
    agents::{Addresses, Payloads},
    postmaster,
};
use chrono::{DateTime, Utc};
use jsy_mk_194_rs::jsy_mk_194g::JsyMk194g;
use jsy_mk_194_rs::types::Baudrate;
use jsy_mk_194_rs::types::Channel;
use jsy_mk_194_rs::types::ChannelStatistics;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tracing::{error, info, warn};

pub struct PowerMeterAgent {
    address: Addresses,
    power_meter: JsyMk194g<SerialStream>,
    receivers: Vec<Addresses>,
    period: Duration,
}

pub struct Config {
    pub serial_port: String,
    pub baud_rate: Baudrate,
    pub period: Duration,
    pub receivers: Vec<Addresses>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PowerReading {
    pub timestamp: DateTime<Utc>,
    pub reading: ChannelStatistics,
}
impl Agent for PowerMeterAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = Config;

    async fn create(address: Self::Address, config: Self::Config) -> Self {
        let port = tokio_serial::new(&config.serial_port, u32::from(config.baud_rate))
            .open_native_async()
            .unwrap();
        let power_meter = JsyMk194g::new_default(port).await.unwrap();
        Self {
            address,
            power_meter,
            receivers: config.receivers,
            period: config.period,
        }
    }

    async fn run(mut self, mut inbox: Inbox<Self::Message>) -> ! {
        let mut ticker = tokio::time::interval(self.period);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    match self.power_meter.get_channel(Channel::One).await {
                        Ok(reading) => {
                            let message = Payloads::PowerReading(PowerReading {
                                timestamp: Utc::now(),
                                reading,
                            });
                            for receiver in &self.receivers {
                                if let Err(err) = postmaster::send( *receiver,self.address, message.clone()).await {
                                    error!(error = ?err, "PowerMeterAgent failed to send reading");
                                }
                            }
                        }
                        Err(err) => {
                            error!(error = ?err, "PowerMeterAgent failed to read meter");
                        }
                    }
                }
                received_message = inbox.recv() => {
                    match received_message {
                        Some(message) => {
                            warn!(payload = ?message.payload, "PowerMeterAgent received an unknown message");
                        }
                        None => {
                            info!("PowerMeterAgent inbox closed");
                        }
                    }
                }
            }
        }
    }
}
