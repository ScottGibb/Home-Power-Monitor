use jsy_mk_194_rs::units::{Energy, Power, kilowatt_hour, watt};

pub trait Screen {
    fn update_display(
        &mut self,
        screen_data: ScreenData,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn get_current_screen(&self) -> ScreenData;
}
#[derive(Debug, Clone, PartialEq)]
pub enum ScreenData {
    // The current running average of power being used
    Average(Power),
    // The current instantaneous power being used
    Instantaneous(Power),
    // The total energy used for the current day and the current power
    Daily {
        current_power: Power,
        energy: Energy,
    },
    // The total energy used for the month, which includes daily average, highest day and the current total
    Monthly {
        total_energy: Energy,
        daily_low: Energy,
        daily_avg: Energy,
        daily_high: Energy,
    },
    // The total energy used for the year
    Yearly {
        lowest_day: Energy,
        avg_day: Energy,
        highest_day: Energy,
        total_energy: Energy,
    },
    Message(ScreenMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenMessage {
    Custom { title: String, content: String },
    Error(String),
}
