use home_power_monitor::postmaster;

#[cfg(not(feature = "mqtt"))]
fn main() {
    panic!(
        "This example requires the 'mqtt' feature to be enabled. Please run with '--features mqtt'."
    );
}

#[cfg(feature = "mqtt")]
#[tokio::main]
async fn main() {
    use home_power_monitor::agents::{Addresses, Payloads};
    use home_power_monitor::agents::{
        exports::mqtt_exporter_agent::{self, MQTTExporterAgent},
        inputs::power_meter_agent::PowerReading,
    };
    use jsy_mk_194_rs::{
        types::{ChannelStatistics, PowerDirection},
        units::{ElectricCurrent, ElectricPotential, Energy, Power, ampere, volt, watt, watt_hour},
    };
    use tracing::info;

    home_power_monitor::init_tracing();

    let config = mqtt_exporter_agent::Config {
        server_address: "127.0.0.1".to_string(),
        port: 1883,
    };

    // Registering the MQTT exporter publishes Home Assistant discovery config.
    postmaster::register_agent!(MQTT, MQTTExporterAgent, config).unwrap();

    let mut watts: f32 = 0.0;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let record = PowerReading {
            timestamp: chrono::Utc::now(),
            reading: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(230.0),
                current: ElectricCurrent::new::<ampere>(0.0),
                active_power: Power::new::<watt>(watts),
                positive_active_energy: Energy::new::<watt_hour>(0.0),
                negative_active_energy: Energy::new::<watt_hour>(0.0),
                power_factor: 1.0,
                power_direction: PowerDirection::Positive,
            },
        };

        postmaster::send(
            Addresses::MQTT,
            Addresses::Core,
            Payloads::PowerReading(record),
        )
        .await
        .unwrap();

        info!(watts = watts, "Published power reading to MQTT");
        watts += 1.0;
    }
}
