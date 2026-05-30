use home_power_monitor::postmaster;

#[cfg(not(feature = "terminal-buttons"))]
fn main() {
    panic!(
        "This example requires the 'terminal-buttons' feature to be enabled. Please run with '--features terminal-buttons'."
    );
}

#[cfg(feature = "terminal-buttons")]
#[tokio::main]
async fn main() {
    use home_power_monitor::agents::{
        Addresses,
        debug_agent::DebugAgent,
        inputs::{
            Button,
            buttons::terminal_command_agent::{TerminalButtonConfig, TerminalCommandAgent},
        },
    };

    home_power_monitor::init_tracing();

    let mut keymap = TerminalButtonConfig::new();
    keymap.insert("a".to_string(), Button::NextScreen);
    keymap.insert("b".to_string(), Button::PreviousScreen);

    let agent = TerminalCommandAgent {
        keymap,
        receivers: vec![Addresses::DebugAgent],
    };
    tokio::spawn(agent.button_task());

    postmaster::register_agent!(DebugAgent, DebugAgent, ()).unwrap();

    // Keep the main task alive to listen for button presses
    tokio::signal::ctrl_c().await.unwrap();
}
