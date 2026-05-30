pub mod debug_agent;
pub mod exports;
pub mod inputs;
pub mod screen;

use crate::agents::{
    inputs::{Button, power_meter_agent::PowerReading},
    screen::screen::ScreenMessage,
};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Payloads {
    ButtonPressed(Button),
    PowerReading(PowerReading),
    ScreenUpdate(ScreenMessage),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
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
