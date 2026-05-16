use home_power_monitor::{
    agents::{
        Addresses, Payloads,
        exports::csv_exporter_agent::{self, CSVExporterAgent},
        inputs::power_meter_agent::PowerReading,
    },
    postmaster,
};
use jsy_mk_194_rs::{
    types::PowerDirection,
    units::{ampere, volt, watt},
};

use jsy_mk_194_rs::{
    types::ChannelStatistics,
    units::{ElectricCurrent, ElectricPotential, Energy, Power, watt_hour},
};

#[cfg(not(feature = "csv"))]
fn main() {
    panic!(
        "This example requires the 'csv' feature to be enabled. Please run with '--features csv'."
    );
}

#[cfg(feature = "csv")]
#[tokio::main]
async fn main() {
    let config = csv_exporter_agent::Config {
        file_path: "power_readings.csv".into(),
    };
    postmaster::register_agent!(CSV, CSVExporterAgent, config).unwrap();

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let record = PowerReading {
            timestamp: chrono::Utc::now(),
            reading: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(rand::random::<f32>() * 230.0),
                current: ElectricCurrent::new::<ampere>(rand::random::<f32>() * 10.0),
                active_power: Power::new::<watt>(rand::random::<f32>() * 2000.0),
                power_factor: 0.95,
                power_direction: PowerDirection::Negative,
                positive_active_energy: Energy::new::<watt_hour>(rand::random::<f32>() * 500.0),
                negative_active_energy: Energy::new::<watt_hour>(rand::random::<f32>() * 500.0),
            },
        };
        println!("Generated random power reading: {}", record.reading);
        postmaster::send(
            Addresses::CSV,
            Addresses::Core,
            Payloads::PowerReading(record),
        )
        .await
        .unwrap();
    }
}
