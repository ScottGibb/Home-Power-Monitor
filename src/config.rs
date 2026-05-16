use std::path::PathBuf;

use crate::agents::{
    Addresses,
    exports::csv_exporter_agent,
    inputs::{
        Button,
        configs::{TerminalButtonConfig, TerminalButtonConfigs},
    },
    power_meter_agent,
};
use jsy_mk_194_rs::types::Baudrate;

pub fn get_terminal_button_configs() -> TerminalButtonConfigs {
    TerminalButtonConfigs::default()
}
pub fn get_power_meter_config() -> power_meter_agent::Config {
    power_meter_agent::Config {
        serial_port: "/dev/ttyUSB0".to_string(),
        baud_rate: Baudrate::default(),
        period: std::time::Duration::from_secs(5),
        receivers: vec![Addresses::CSV],
    }
}

pub fn get_csv_exporter_config() -> csv_exporter_agent::Config {
    csv_exporter_agent::Config {
        file_path: PathBuf::from("power_readings.csv"),
    }
}

impl Default for TerminalButtonConfigs {
    fn default() -> Self {
        TerminalButtonConfigs::new(vec![TerminalButtonConfig {
            key: "start",
            button: Button::Start,
            receivers: vec![Addresses::DebugAgent],
        }])
    }
}
