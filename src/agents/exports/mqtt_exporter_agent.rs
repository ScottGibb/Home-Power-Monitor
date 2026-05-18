use crate::{
    agents::{
        Addresses, Payloads, exports::home_assistant_protocol::HomeAssistantSensorDiscovery,
        inputs::power_meter_agent::PowerReading,
    },
    postmaster::{self, Message},
};
use jsy_mk_194_rs::units::watt;
use mqtt_endpoint_tokio::mqtt_ep::{
    self, ConnectionError, Endpoint, Mode, Version,
    packet::{
        Packet, Qos,
        v3_1_1::{Connect, Publish},
    },
    role::Client,
    transport::TcpTransport,
};
use post_haste::agent::{Agent, Inbox};
use tracing::{error, info, warn};

type ClientEndpoint = Endpoint<Client>;

const DISCOVERY_TOPIC: &str = "homeassistant/sensor/home_power_monitor_active_power/config";
const STATE_TOPIC: &str = "home_power_monitor/power/active_w";

pub struct MQTTExporterAgent {
    _address: Addresses,
    endpoint: ClientEndpoint,
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
        let endpoint = Self::create_endpoint(&config).await;

        let agent = Self {
            _address: address,
            endpoint,
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
    async fn create_endpoint(config: &Config) -> ClientEndpoint {
        let endpoint = ClientEndpoint::new(Version::V3_1_1);

        let tcp_stream =
            tokio::net::TcpStream::connect(format!("{}:{}", config.server_address, config.port))
                .await
                .expect("Failed to connect to MQTT broker");

        let transport = TcpTransport::from_stream(tcp_stream);
        endpoint
            .attach(transport, Mode::Client)
            .await
            .expect("Failed to attach MQTT transport");

        let connect_packet = Connect::builder()
            .client_id("home_power_monitor")
            .expect("Invalid MQTT client id")
            .keep_alive(60)
            .clean_session(true)
            .build()
            .expect("Failed to build MQTT connect packet");
        endpoint
            .send(connect_packet)
            .await
            .expect("Failed to send MQTT connect packet");

        match endpoint.recv().await {
            Ok(Packet::V3_1_1Connack(connack)) => {
                if connack.return_code() != mqtt_ep::result_code::ConnectReturnCode::Accepted {
                    panic!("MQTT connection refused: {:?}", connack.return_code());
                }
            }
            Ok(packet) => panic!("Expected MQTT CONNACK, got {:?}", packet.packet_type()),
            Err(err) => panic!("Failed to receive MQTT CONNACK: {:?}", err),
        }

        endpoint
    }

    async fn publish_discovery(&self) -> Result<(), ConnectionError> {
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

    async fn publish_power_state(&self, reading: &PowerReading) -> Result<(), ConnectionError> {
        let payload = reading.reading.active_power.get::<watt>().to_string();
        self.publish(STATE_TOPIC, &payload, false).await
    }

    async fn publish(
        &self,
        topic: &str,
        payload: &str,
        retain: bool,
    ) -> Result<(), ConnectionError> {
        let publish_packet = Publish::builder()
            .topic_name(topic)
            .expect("Invalid MQTT topic")
            .qos(Qos::AtMostOnce)
            .retain(retain)
            .payload(payload.as_bytes())
            .build()
            .expect("Failed to build MQTT publish packet");

        self.endpoint.send(publish_packet).await
    }
}
