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
    postmaster,
};

pub struct ScreenAgent<S: Screen> {
    _address: Addresses,
    screen: S,
    current_power: Power,
    average_power: Power,
}

pub struct Config<S: Screen> {
    pub screen: S,
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
        }
    }

    async fn run(mut self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            match received_message.payload {
                Payloads::ButtonPressed(button) => {
                    let current_screen = self.screen.get_current_screen();
                    let new_screen = match button {
                        Button::NextSreen => self.next(current_screen),
                        Button::PreviousScreen => self.previous(current_screen),
                    };
                    self.screen.update_display(new_screen).await;
                }
                Payloads::PowerReading(reading) => {
                    // Update Internal Power State
                    self.current_power = reading.reading.active_power;
                    match self.screen.get_current_screen() {
                        ScreenData::Instantaneous(_) | ScreenData::Error(_) => {
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
                energy: Energy::default(),
            },
            ScreenData::Daily { .. } => ScreenData::Monthly {
                total_energy: Energy::default(),
                daily_low: Energy::default(),
                daily_avg: Energy::default(),
                daily_high: Energy::default(),
            },
            ScreenData::Monthly { .. } => ScreenData::Yearly {
                lowest_day: Energy::default(),
                avg_day: Energy::default(),
                highest_day: Energy::default(),
                total_energy: Energy::default(),
            },
            ScreenData::Yearly { .. } => ScreenData::Instantaneous(self.current_power),
            _ => current_screen,
        }
    }
    pub fn previous(&self, current_screen: ScreenData) -> ScreenData {
        match current_screen {
            ScreenData::Average(_) => ScreenData::Instantaneous(self.current_power),
            ScreenData::Instantaneous(_) => ScreenData::Yearly {
                lowest_day: Energy::default(),
                avg_day: Energy::default(),
                highest_day: Energy::default(),
                total_energy: Energy::default(),
            },
            ScreenData::Daily { .. } => ScreenData::Average(self.average_power),
            ScreenData::Monthly { .. } => ScreenData::Daily {
                current_power: self.current_power,
                energy: Energy::default(),
            },
            ScreenData::Yearly { .. } => ScreenData::Monthly {
                total_energy: Energy::default(),
                daily_low: Energy::default(),
                daily_avg: Energy::default(),
                daily_high: Energy::default(),
            },
            _ => current_screen,
        }
    }
}
