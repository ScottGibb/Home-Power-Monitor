pub mod buttons;
pub mod polite_agent;
pub mod power_meter_agent;
pub use power_meter_agent::PowerMeterAgent;

use crate::agents::buttons::Button;

#[derive(Debug, Clone, Copy)]
pub enum Payloads {
    Hello,
    ButtonPressed(Button),
}

#[derive(Debug, Clone, Copy)]
pub enum Addresses {
    Core,
    PoliteAgent,
    PowerMeter,
    Screen,
    Button,
}
