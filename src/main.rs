use home_power_monitor::{print_metadata, setup_agents, setup_terminal_buttons};

#[tokio::main]
async fn main() {
    setup_agents().await;
    #[cfg(feature = "terminal-buttons")]
    setup_terminal_buttons().await;
    #[cfg(feature = "gpio-buttons")]
    setup_gpio_buttons().await;
    print_metadata();

    tokio::signal::ctrl_c().await.unwrap();
}
