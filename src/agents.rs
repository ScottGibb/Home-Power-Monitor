pub mod debug_agent;
pub mod exports;
pub mod inputs;
pub mod power_meter_agent;

use crate::agents::{inputs::Button, power_meter_agent::PowerReading};

#[derive(Debug, Clone, PartialEq)]
pub enum Payloads {
    ButtonPressed(Button),
    PowerReading(PowerReading),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Addresses {
    Core,
    PoliteAgent,
    Button,
    PowerMeter,
    CSV,
    Database,
    MQTT,
    Screen,
}
