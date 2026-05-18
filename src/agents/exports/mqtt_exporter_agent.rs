use crate::{
    agents::{
        Addresses, Payloads, exports::home_assistant_protocol::HomeAssistantSensorDiscovery,
        inputs::power_meter_agent::PowerReading,
    },
    postmaster::{self, Message},
};
use jsy_mk_194_rs::units::watt;
use post_haste::agent::{Agent, Inbox};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tracing::{error, info, warn};

const DISCOVERY_TOPIC: &str = "homeassistant/sensor/home_power_monitor_active_power/config";
const STATE_TOPIC: &str = "home_power_monitor/power/active_w";

pub struct MQTTExporterAgent {
    _address: Addresses,
    client: AsyncClient,
}

pub struct Config {
    pub server_address: String,
    pub port: u16,
}

impl Agent for MQTTExporterAgent {
    type Address = Addresses;
    type Message = Message;
    type Config = Config;

    async fn create(address: Self::Address, config: Self::Config) -> Self {
        let (client, mut eventloop) = Self::create_client(&config);

        // Spawn event loop to handle connection in background
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {}
                    Err(e) => {
                        error!(error = ?e, "MQTT connection error");
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });

        let agent = Self {
            _address: address,
            client,
        };

        match agent.publish_discovery().await {
            Ok(_) => {
                info!("Published Home Assistant MQTT discovery config");
            }
            Err(err) => {
                error!(error = ?err, "Failed to publish Home Assistant MQTT discovery");
            }
        }

        agent
    }

    async fn run(self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await;
            match received_message {
                Some(message) => match &message.payload {
                    Payloads::PowerReading(reading) => {
                        if let Err(err) = self.publish_power_state(reading).await {
                            error!(error = ?err, "Failed to publish power state to MQTT");
                        }
                    }
                    _ => {
                        warn!(payload = ?message.payload, "MQTTExporterAgent received unexpected message");
                    }
                },
                None => {
                    warn!("MQTTExporterAgent inbox closed");
                }
            }
        }
    }
}

impl MQTTExporterAgent {
    fn create_client(config: &Config) -> (AsyncClient, rumqttc::EventLoop) {
        let mut mqttoptions =
            MqttOptions::new("home_power_monitor", &config.server_address, config.port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(60));
        mqttoptions.set_clean_session(true);

        AsyncClient::new(mqttoptions, 10)
    }

    async fn publish_discovery(&self) -> Result<(), rumqttc::ClientError> {
        let payload = HomeAssistantSensorDiscovery::new(
            "Home Power Active Power",
            "home_power_monitor",
            STATE_TOPIC,
            "W",
            "power",
            "measurement",
            "home_power_monitor",
            "Home Power Monitor",
            "home_power_monitor",
        )
        .to_json_string();
        self.publish(DISCOVERY_TOPIC, &payload, true).await
    }

    async fn publish_power_state(
        &self,
        reading: &PowerReading,
    ) -> Result<(), rumqttc::ClientError> {
        let payload = reading.reading.active_power.get::<watt>().to_string();
        self.publish(STATE_TOPIC, &payload, false).await
    }

    async fn publish(
        &self,
        topic: &str,
        payload: &str,
        retain: bool,
    ) -> Result<(), rumqttc::ClientError> {
        self.client
            .publish(topic, QoS::AtMostOnce, retain, payload.as_bytes())
            .await
    }
}
