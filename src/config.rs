use crate::agents::buttons::configs::TerminalButtonConfigs;

pub fn read_terminal_button_configs() -> TerminalButtonConfigs {
    // For simplicity, we hardcode the button configurations here.
    // In a real application, you might read this from a config file or environment variables.
    TerminalButtonConfigs::default()
}
