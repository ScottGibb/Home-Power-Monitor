#![allow(unused_imports)]
#![feature(variant_count)]

commitment_issues::include_metadata!();
pub mod agents;
pub mod config;

#[cfg(feature = "mqtt")]
use crate::agents::exports::mqtt_exporter_agent::MQTTExporterAgent;

#[cfg(feature = "mock-power-meter")]
use crate::agents::inputs::mock_power_meter_agent::MockPowerMeterAgent;

#[cfg(feature = "mqtt")]
use crate::config::get_mqtt_exporter_config;

#[cfg(feature = "csv")]
use crate::{
    agents::exports::csv_exporter_agent::CSVExporterAgent, config::get_csv_exporter_config,
};

#[cfg(feature = "mock-power-meter")]
use crate::agents::inputs::mock_power_meter_agent::Config;

use crate::agents::{
    Addresses,
    debug_agent::DebugAgent,
    inputs::{
        buttons::terminal_command_agent::TerminalCommandAgent, power_meter_agent::PowerMeterAgent,
    },
};
use crate::config::{get_power_meter_config, get_terminal_button_configs};
use post_haste::init_postmaster;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::agents::Payloads;
init_postmaster!(Addresses, Payloads);

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}

pub fn print_metadata() {
    info!("Here is the binary metadata");
    info!(schema_version = %metadata::schema(), "metadata");
    info!(compile_time = %metadata::compile_time(), "metadata");
    info!(commit_hash = %metadata::short_hash(), "metadata");
    info!(is_dirty_build = %metadata::is_dirty(), "metadata");
    info!(tag_description = %metadata::tag_describe(), "metadata");
    info!(last_author = %metadata::last_author(), "metadata");
}

pub async fn setup_agents() {
    postmaster::register_agent!(DebugAgent, DebugAgent, ()).unwrap();
    #[cfg(not(feature = "mock-power-meter"))]
    postmaster::register_agent!(PowerMeter, PowerMeterAgent, get_power_meter_config()).unwrap();
    #[cfg(feature = "mock-power-meter")]
    postmaster::register_agent!(
        PowerMeter,
        MockPowerMeterAgent,
        Config {
            period: get_power_meter_config().period,
            receivers: get_power_meter_config().receivers,
        }
    )
    .unwrap();
    #[cfg(feature = "csv")]
    postmaster::register_agent!(CSV, CSVExporterAgent, get_csv_exporter_config()).unwrap();
    #[cfg(feature = "mqtt")]
    postmaster::register_agent!(MQTT, MQTTExporterAgent, get_mqtt_exporter_config()).unwrap();
    #[cfg(feature = "database")]
    unimplemented!("Database agent is not yet implemented");
    #[cfg(feature = "screen")]
    unimplemented!("Screen agent is not yet implemented");
}

pub async fn setup_terminal_buttons() {
    let buttons = get_terminal_button_configs();
    for button in buttons {
        let agent = TerminalCommandAgent {
            key: button.key,
            button: button.button,
            receivers: button.receivers,
        };
        tokio::spawn(agent.button_task());
    }
}

pub async fn setup_gpio_buttons() {
    unimplemented!("GPIO button support is not yet implemented");
}
