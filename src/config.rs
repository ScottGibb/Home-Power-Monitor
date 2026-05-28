use std::path::PathBuf;

#[cfg(feature = "csv")]
use crate::agents::exports::csv_exporter_agent;
#[cfg(feature = "mqtt")]
use crate::agents::exports::mqtt_exporter_agent;
#[cfg(feature = "screen")]
use crate::agents::screen::{
    mock_screen::MockScreen,
    screen::{Screen, ScreenData},
    screen_agent,
};
use crate::agents::{
    Addresses,
    inputs::{
        Button,
        buttons::configs::{TerminalButtonConfig, TerminalButtonConfigs},
        power_meter_agent,
    },
};
use jsy_mk_194_rs::types::Baudrate;

pub fn get_terminal_button_configs() -> TerminalButtonConfigs {
    TerminalButtonConfigs::default()
}

pub fn get_power_meter_config() -> power_meter_agent::Config {
    let mut receivers = Vec::new();
    #[cfg(feature = "csv")]
    receivers.push(Addresses::CSV);
    #[cfg(feature = "mqtt")]
    receivers.push(Addresses::MQTT);

    power_meter_agent::Config {
        serial_port: "/dev/tty.usbserial-0001".to_string(),
        baud_rate: Baudrate::default(),
        period: std::time::Duration::from_secs(5),
        receivers,
    }
}

#[cfg(feature = "csv")]
pub fn get_csv_exporter_config() -> csv_exporter_agent::Config {
    csv_exporter_agent::Config {
        file_path: PathBuf::from("power_readings.csv"),
    }
}

#[cfg(feature = "mqtt")]
pub fn get_mqtt_exporter_config() -> mqtt_exporter_agent::Config {
    mqtt_exporter_agent::Config {
        server_address: "127.0.0.1".to_string(),
        port: 1883,
    }
}
#[cfg(feature = "screen")]
pub fn get_screen_agent_config() -> screen_agent::Config<MockScreen> {
    let screen = MockScreen::new();
    screen_agent::Config { screen }
}

impl Default for TerminalButtonConfigs {
    fn default() -> Self {
        TerminalButtonConfigs::new(vec![
            TerminalButtonConfig {
                key: "next",
                button: Button::NextSreen,
                receivers: vec![Addresses::Screen],
            },
            TerminalButtonConfig {
                key: "previous",
                button: Button::PreviousScreen,
                receivers: vec![Addresses::Screen],
            },
        ])
    }
}
