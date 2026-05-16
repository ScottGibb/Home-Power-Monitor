use home_power_monitor::agents::debug_agent::DebugAgent;
use home_power_monitor::agents::inputs::buttons::terminal_command_agent::TerminalCommandAgent;
use home_power_monitor::agents::{Addresses, inputs::Button};
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
    let agent = TerminalCommandAgent {
        key: "a",
        button: Button::Start,
        receivers: vec![Addresses::DebugAgent],
    };
    tokio::spawn(agent.button_task());

    postmaster::register_agent!(DebugAgent, DebugAgent, ()).unwrap();

    // Keep the main task alive to listen for button presses
    tokio::signal::ctrl_c().await.unwrap();
}
