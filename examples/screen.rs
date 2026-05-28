use home_power_monitor::{
    agents::{
        Payloads,
        inputs::power_meter_agent::PowerReading,
        screen::screen_agent::{Config, ScreenAgent},
    },
    postmaster,
};
use jsy_mk_194_rs::{
    types::PowerDirection,
    units::{ElectricCurrent, Energy, Power, ampere, kilowatt_hour, watt},
};

#[cfg(not(all(feature = "terminal-buttons", feature = "screen")))]
fn main() {
    panic!(
        "This example requires the 'terminal-buttons' or 'screen' feature to be enabled. Please run with '--features terminal-buttons' or '--features screen'."
    );
}

#[cfg(all(feature = "terminal-buttons", feature = "screen"))]
#[tokio::main]
async fn main() {
    use home_power_monitor::agents::{
        Addresses, inputs::Button, inputs::buttons::terminal_command_agent::TerminalCommandAgent,
        screen::mock_screen::MockScreen,
    };
    use jsy_mk_194_rs::{
        types::ChannelStatistics,
        units::{ElectricPotential, volt},
    };

    home_power_monitor::init_tracing();

    let next_button = TerminalCommandAgent {
        key: "next",
        button: Button::NextSreen,
        receivers: vec![Addresses::Screen],
    };
    let previous_button = TerminalCommandAgent {
        key: "previous",
        button: Button::PreviousScreen,
        receivers: vec![Addresses::Screen],
    };
    tokio::spawn(next_button.button_task());
    tokio::spawn(previous_button.button_task());

    let screen = MockScreen::new();
    postmaster::register_agent!(Screen, ScreenAgent<MockScreen>, Config { screen }).unwrap();

    let mut active_power_w = 0.0_f32;
    loop {
        active_power_w = (active_power_w + 10.0) % 3000.0;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        postmaster::send(
            Addresses::Screen,
            Addresses::PowerMeter,
            Payloads::PowerReading(PowerReading {
                timestamp: chrono::Utc::now(),
                reading: ChannelStatistics {
                    voltage: ElectricPotential::new::<volt>(230.0),
                    current: ElectricCurrent::new::<ampere>(active_power_w / 230.0),
                    active_power: Power::new::<watt>(active_power_w),
                    positive_active_energy: Energy::new::<kilowatt_hour>(1.0),
                    negative_active_energy: Energy::new::<kilowatt_hour>(0.0),
                    power_factor: 1.0,
                    power_direction: PowerDirection::Positive,
                },
            }),
        )
        .await
        .unwrap();
    }
}
