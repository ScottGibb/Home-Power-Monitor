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
    Error(String),
}

impl ToString for ScreenData {
    fn to_string(&self) -> String {
        match self {
            ScreenData::Average(power) => format!("Average Power: {:.2} W", power.get::<watt>()),
            ScreenData::Instantaneous(power) => {
                format!("Instantaneous Power: {:.2} W", power.get::<watt>())
            }
            ScreenData::Daily {
                current_power,
                energy,
            } => format!(
                "Daily Energy: {:.2} kWh, Current Power: {:.2} W",
                energy.get::<kilowatt_hour>(),
                current_power.get::<watt>()
            ),
            ScreenData::Monthly {
                total_energy,
                daily_avg,
                daily_high,
                ..
            } => format!(
                "Monthly Energy: {:.2} kWh, Daily Avg: {:.2} kWh, Daily High: {:.2} kWh",
                total_energy.get::<kilowatt_hour>(),
                daily_avg.get::<kilowatt_hour>(),
                daily_high.get::<kilowatt_hour>()
            ),
            ScreenData::Yearly { total_energy, .. } => {
                format!(
                    "Yearly Energy: {:.2} kWh",
                    total_energy.get::<kilowatt_hour>()
                )
            }
            ScreenData::Error(err) => format!("Error: {}", err),
        }
    }
}
