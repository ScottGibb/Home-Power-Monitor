use crate::agents::{Addresses, Button};

pub struct TerminalButtonConfig {
    pub key: &'static str,
    pub button: Button,
    pub receivers: Vec<Addresses>,
}

pub struct TerminalButtonConfigs(Vec<TerminalButtonConfig>);

impl Default for TerminalButtonConfigs {
    fn default() -> Self {
        TerminalButtonConfigs(vec![TerminalButtonConfig {
            key: "start",
            button: Button::Start,
            receivers: vec![Addresses::PoliteAgent],
        }])
    }
}

impl IntoIterator for TerminalButtonConfigs {
    type Item = TerminalButtonConfig;
    type IntoIter = std::vec::IntoIter<TerminalButtonConfig>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
