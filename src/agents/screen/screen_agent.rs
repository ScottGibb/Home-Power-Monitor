use jsy_mk_194_rs::units::{Energy, Power};
use post_haste::agent::{Agent, Inbox};
use tracing::info;

use crate::{
    Addresses,
    agents::{
        Payloads,
        inputs::Button,
        screen::{
            self,
            screen::{Screen, ScreenData},
        },
    },
    database::Database,
    postmaster,
};

pub struct ScreenAgent<S: Screen> {
    _address: Addresses,
    screen: S,
    database: Database,
    current_power: Power,
    average_power: Power,
}

pub struct Config<S: Screen> {
    pub screen: S,
    pub database: Database,
}
impl<S: Screen> Agent for ScreenAgent<S> {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = Config<S>;

    async fn create(_address: Self::Address, config: Self::Config) -> Self {
        Self {
            _address,
            screen: config.screen,
            current_power: Power::default(),
            average_power: Power::default(),
            database: config.database,
        }
    }

    async fn run(mut self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            info!(
                "ScreenAgent received message: {:?}",
                received_message.payload
            );
            match received_message.payload {
                Payloads::ButtonPressed(button) => {
                    let current_screen = self.screen.get_current_screen();
                    let new_screen = match button {
                        Button::NextScreen => self.next(current_screen),
                        Button::PreviousScreen => self.previous(current_screen),
                    };
                    info!("Updating screen to: {:?}", new_screen);
                    self.screen.update_display(new_screen).await;
                }
                Payloads::ScreenUpdate(message) => {
                    self.screen
                        .update_display(ScreenData::Message(message))
                        .await;
                }
                Payloads::PowerReading(reading) => {
                    // Update Internal Power State
                    self.current_power = reading.reading.active_power;
                    match self.screen.get_current_screen() {
                        ScreenData::Instantaneous(_) => {
                            self.screen
                                .update_display(ScreenData::Instantaneous(self.current_power))
                                .await;
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

impl<S: Screen> ScreenAgent<S> {
    pub fn next(&self, current_screen: ScreenData) -> ScreenData {
        match current_screen {
            ScreenData::Instantaneous(_) => ScreenData::Average(self.average_power),
            ScreenData::Average(_) => ScreenData::Daily {
                current_power: self.current_power,
                energy: self.database.get_daily_energy().avg,
            },
            ScreenData::Daily { .. } => ScreenData::Monthly {
                total_energy: self.database.get_monthly_energy().total_energy,
                daily_low: self.database.get_monthly_energy().daily_low,
                daily_avg: self.database.get_monthly_energy().daily_avg,
                daily_high: self.database.get_monthly_energy().daily_high,
            },
            ScreenData::Monthly { .. } => ScreenData::Yearly {
                lowest_day: self.database.get_yearly_energy().lowest_day,
                avg_day: self.database.get_yearly_energy().avg_day,
                highest_day: self.database.get_yearly_energy().highest_day,
                total_energy: self.database.get_yearly_energy().total_energy,
            },
            ScreenData::Yearly { .. } => ScreenData::Instantaneous(self.current_power),
            _ => ScreenData::Instantaneous(self.current_power),
        }
    }
    pub fn previous(&self, current_screen: ScreenData) -> ScreenData {
        match current_screen {
            ScreenData::Average(_) => ScreenData::Instantaneous(self.current_power),
            ScreenData::Instantaneous(_) => ScreenData::Yearly {
                lowest_day: self.database.get_yearly_energy().lowest_day,
                avg_day: self.database.get_yearly_energy().avg_day,
                highest_day: self.database.get_yearly_energy().highest_day,
                total_energy: self.database.get_yearly_energy().total_energy,
            },
            ScreenData::Daily { .. } => ScreenData::Average(self.average_power),
            ScreenData::Monthly { .. } => ScreenData::Daily {
                current_power: self.current_power,
                energy: self.database.get_daily_energy().avg,
            },
            ScreenData::Yearly { .. } => ScreenData::Monthly {
                total_energy: self.database.get_monthly_energy().total_energy,
                daily_low: self.database.get_monthly_energy().daily_low,
                daily_avg: self.database.get_monthly_energy().daily_avg,
                daily_high: self.database.get_monthly_energy().daily_high,
            },
            _ => ScreenData::Instantaneous(self.current_power),
        }
    }
}
