use home_power_monitor::{init_tracing, print_metadata, setup_agents};

#[tokio::main]
async fn main() {
    init_tracing();
    setup_agents().await;
    #[cfg(feature = "terminal-buttons")]
    home_power_monitor::setup_terminal_buttons().await;
    #[cfg(feature = "gpio-buttons")]
    home_power_monitor::setup_gpio_buttons().await;
    print_metadata();

    tokio::signal::ctrl_c().await.unwrap();
}
