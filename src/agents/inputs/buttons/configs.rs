use crate::agents::{Addresses, Button};

pub struct TerminalButtonConfig {
    pub key: &'static str,
    pub button: Button,
    pub receivers: Vec<Addresses>,
}

pub struct TerminalButtonConfigs(Vec<TerminalButtonConfig>);

impl TerminalButtonConfigs {
    pub fn new(configs: Vec<TerminalButtonConfig>) -> Self {
        Self(configs)
    }
}

impl IntoIterator for TerminalButtonConfigs {
    type Item = TerminalButtonConfig;
    type IntoIter = std::vec::IntoIter<TerminalButtonConfig>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
