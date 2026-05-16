#![feature(variant_count)]

commitment_issues::include_metadata!();
pub mod agents;
pub mod config;
use crate::{
    agents::{
        Addresses, Payloads, PowerMeterAgent, buttons::terminal_button_agent::TerminalButtonAgent,
        debug_agent::DebugAgent,
    },
    config::{get_power_meter_config, get_terminal_button_configs},
};
use post_haste::init_postmaster;
init_postmaster!(Addresses, Payloads);

pub fn print_metadata() {
    println!("\n Here is the binary's metadata:\n");
    println!("Schema version:  {}", metadata::schema());
    println!("Compile time:    {}", metadata::compile_time());
    println!("Commit hash:     {}", metadata::short_hash());
    println!("Is dirty build:  {}", metadata::is_dirty());
    println!("Tag description: {}", metadata::tag_describe());
    println!("Last author:     {}", metadata::last_author());
}

pub async fn setup_agents() {
    postmaster::register_agent!(PoliteAgent, DebugAgent, ()).unwrap();
    postmaster::register_agent!(PowerMeter, PowerMeterAgent, get_power_meter_config()).unwrap();
}

pub async fn setup_terminal_buttons() {
    let buttons = get_terminal_button_configs();
    for button in buttons {
        let agent = TerminalButtonAgent {
            key: button.key,
            button: button.button,
            receivers: button.receivers,
        };
        tokio::spawn(agent.button_task());
    }
}
