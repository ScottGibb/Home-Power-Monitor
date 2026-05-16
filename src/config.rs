use crate::agents::{
    Addresses,
    buttons::{
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
        receivers: vec![Addresses::PoliteAgent],
    }
}

impl Default for TerminalButtonConfigs {
    fn default() -> Self {
        TerminalButtonConfigs::new(vec![TerminalButtonConfig {
            key: "start",
            button: Button::Start,
            receivers: vec![Addresses::PoliteAgent],
        }])
    }
}
