pub mod debug_agent;
pub mod exports;
pub mod inputs;

use crate::agents::inputs::{Button, power_meter_agent::PowerReading};

#[derive(Debug, Clone, PartialEq)]
pub enum Payloads {
    ButtonPressed(Button),
    PowerReading(PowerReading),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Addresses {
    Core,
    DebugAgent,
    Button,
    PowerMeter,
    CSV,
    Database,
    MQTT,
    Screen,
}
