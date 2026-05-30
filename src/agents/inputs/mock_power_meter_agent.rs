use std::time::Duration;

use chrono::Utc;
use jsy_mk_194_rs::{
    types::{ChannelStatistics, PowerDirection},
    units::{ElectricCurrent, ElectricPotential, Energy, Power, ampere, volt, watt, watt_hour},
};
use post_haste::agent::{Agent, Inbox};
use tracing::{error, info, warn};

use crate::agents::inputs::power_meter_agent::Config;
use crate::{
    agents::{
        Addresses, Payloads,
        inputs::power_meter_agent::{PowerMeterAgent, PowerReading},
    },
    postmaster,
};

pub struct MockPowerMeterAgent {
    address: Addresses,
    receivers: Vec<Addresses>,
    period: Duration,
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
                    // Add a moving element: active_power varies as a sine wave with time
                    let now = Utc::now();
                    let seconds = now.timestamp() as f64 + (now.timestamp_subsec_micros() as f64) / 1_000_000.0;
                    let amplitude = 1000.0; // watts
                    let freq = 1.0 / 60.0; // 1 cycle per minute
                    let active_power_val = amplitude * (2.0 * std::f64::consts::PI * freq * seconds).sin() as f32;

                    let zero_reading = Payloads::PowerReading(PowerReading {
                        timestamp: now,
                        reading: ChannelStatistics {
                            voltage: ElectricPotential::new::<volt>(230.0),
                            current: ElectricCurrent::new::<ampere>(active_power_val / 230.0),
                            active_power: Power::new::<watt>(active_power_val),
                            positive_active_energy: Energy::new::<watt_hour>(0.0),
                            negative_active_energy: Energy::new::<watt_hour>(0.0),
                            power_factor: 1.0,
                            power_direction: if active_power_val >= 0.0 {
                                PowerDirection::Positive
                            } else {
                                PowerDirection::Negative
                            },
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
