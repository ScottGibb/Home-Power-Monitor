pub mod buttons;
pub mod mock_power_meter_agent;
pub mod power_meter_agent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    NextScreen,
    PreviousScreen,
}
