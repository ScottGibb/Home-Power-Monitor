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
use jsy_mk_194_rs::{delay::StdDelay, types::ChannelStatistics};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

pub struct PowerMeterAgent {
    address: Addresses,
    power_meter: JsyMk194g<SerialStream, StdDelay>,
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
        let power_meter = JsyMk194g::new_default(port, StdDelay).await.unwrap();
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
                                if let Err(err) = postmaster::send(self.address, *receiver, message.clone()).await {
                                    eprintln!("PowerMeterAgent failed to send reading: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("PowerMeterAgent failed to read meter: {:?}", err);
                        }
                    }
                }
                received_message = inbox.recv() => {
                    match received_message {
                        Some(message) => {
                            println!(
                                "PowerMeterAgent received an unknown message: {:?}",
                                message.payload
                            );
                        }
                        None => {
                            eprintln!("PowerMeterAgent inbox closed");
                        }
                    }
                }
            }
        }
    }
}
