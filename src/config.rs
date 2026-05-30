use std::path::PathBuf;

#[cfg(feature = "csv")]
use crate::agents::exports::csv_exporter_agent;
#[cfg(feature = "mqtt")]
use crate::agents::exports::mqtt_exporter_agent;
use crate::agents::{
    Addresses,
    inputs::{
        Button,
        buttons::terminal_command_agent::{TerminalButtonConfig, TerminalCommandAgent},
        power_meter_agent,
    },
};
#[cfg(feature = "screen")]
use crate::{
    agents::screen::{
        mock_screen::MockScreen,
        screen::{Screen, ScreenData},
        screen_agent,
    },
    database::Database,
};
use jsy_mk_194_rs::types::Baudrate;

pub fn get_terminal_button_configs() -> TerminalButtonConfig {
    let mut key_map = TerminalButtonConfig::new();
    key_map.insert("a".to_string(), Button::NextScreen);
    key_map.insert("b".to_string(), Button::PreviousScreen);
    key_map
}

pub fn get_power_meter_config() -> power_meter_agent::Config {
    let mut receivers = Vec::new();
    #[cfg(feature = "csv")]
    receivers.push(Addresses::CSV);
    #[cfg(feature = "mqtt")]
    receivers.push(Addresses::MQTT);
    #[cfg(feature = "screen")]
    receivers.push(Addresses::Screen);
    power_meter_agent::Config {
        serial_port: "/dev/tty.usbserial-0001".to_string(),
        baud_rate: Baudrate::default(),
        period: std::time::Duration::from_secs(1),
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
pub async fn get_screen_agent_config() -> screen_agent::Config<MockScreen> {
    let screen = MockScreen::new().await;
    screen_agent::Config {
        screen,
        database: Database {},
    }
}
