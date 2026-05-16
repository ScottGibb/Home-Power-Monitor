use crate::agents::{
    Addresses,
    buttons::{
        Button,
        configs::{TerminalButtonConfig, TerminalButtonConfigs},
    },
};

pub fn read_terminal_button_configs() -> TerminalButtonConfigs {
    // For simplicity, we hardcode the button configurations here.
    // In a real application, you might read this from a config file or environment variables.
    TerminalButtonConfigs::default()
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
