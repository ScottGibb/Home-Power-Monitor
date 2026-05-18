use rumqttc::{AsyncClient, MqttOptions};
use tokio::task;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("rust-test", "192.168.0.68", 1883);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    info!("Connecting to MQTT broker at 192.168.0.68:1883");
    let (client, mut connection) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("#", rumqttc::QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        loop {
            match connection.poll().await {
                Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(p))) => {
                    info!(
                        "Received: topic={}, payload={}",
                        p.topic,
                        String::from_utf8_lossy(&p.payload)
                    );
                }
                Ok(_) => {}
                Err(e) => {
                    error!(error = ?e, "MQTT connection error");
                    break;
                }
            }
        }
    })
    .await
    .unwrap();
}
