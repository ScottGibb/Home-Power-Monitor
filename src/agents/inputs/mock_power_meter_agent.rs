use std::time::Duration;

use chrono::Utc;
use jsy_mk_194_rs::{
    types::{ChannelStatistics, PowerDirection},
    units::{ElectricCurrent, ElectricPotential, Energy, Power, ampere, volt, watt, watt_hour},
};
use post_haste::agent::{Agent, Inbox};
use tracing::{error, info, warn};

use crate::{
    agents::{Addresses, Payloads, inputs::power_meter_agent::PowerReading},
    postmaster,
};

pub struct MockPowerMeterAgent {
    address: Addresses,
    receivers: Vec<Addresses>,
    period: Duration,
}

pub struct Config {
    pub period: Duration,
    pub receivers: Vec<Addresses>,
}

impl Agent for MockPowerMeterAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = Config;

    async fn create(address: Self::Address, config: Self::Config) -> Self {
        Self {
            address,
            receivers: config.receivers,
            period: config.period,
        }
    }

    async fn run(self, mut inbox: Inbox<Self::Message>) -> ! {
        let mut ticker = tokio::time::interval(self.period);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let zero_reading = Payloads::PowerReading(PowerReading {
                        timestamp: Utc::now(),
                        reading: ChannelStatistics {
                            voltage: ElectricPotential::new::<volt>(0.0),
                            current: ElectricCurrent::new::<ampere>(0.0),
                            active_power: Power::new::<watt>(0.0),
                            positive_active_energy: Energy::new::<watt_hour>(0.0),
                            negative_active_energy: Energy::new::<watt_hour>(0.0),
                            power_factor: 0.0,
                            power_direction: PowerDirection::Positive,
                        },
                    });

                    for receiver in &self.receivers {
                        if let Err(err) = postmaster::send(*receiver, self.address, zero_reading.clone()).await {
                            error!(error = ?err, "MockPowerMeterAgent failed to send reading");
                        }
                    }
                }
                received_message = inbox.recv() => {
                    match received_message {
                        Some(message) => {
                            warn!(
                                payload = ?message.payload,
                                "MockPowerMeterAgent received an unknown message"
                            );
                        }
                        None => {
                            info!("MockPowerMeterAgent inbox closed");
                        }
                    }
                }
            }
        }
    }
}
