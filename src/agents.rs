pub mod buttons;
pub mod debug_agent;
pub mod power_meter_agent;
use jsy_mk_194_rs::types::ChannelStatistics;
pub use power_meter_agent::PowerMeterAgent;

use crate::agents::buttons::Button;

#[derive(Debug, Clone, PartialEq)]
pub enum Payloads {
    ButtonPressed(Button),
    PowerReading(ChannelStatistics),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Addresses {
    Core,
    PoliteAgent,
    PowerMeter,
    Screen,
    Button,
}
