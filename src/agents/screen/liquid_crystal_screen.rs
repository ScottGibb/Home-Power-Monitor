use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use liquid_crystal::{Async, BusBits, Layout, LiquidCrystal, Parallel};

use crate::agents::screen::screen::{Screen, ScreenData, ScreenMessage};

const SCREEN_WIDTH: u8 = 20;
const SCREEN_HEIGHT: usize = 2;

pub struct LiquidCrystalScreen<T: OutputPin> {
    current_screen: ScreenData,
    interface: Parallel<T, T, T, T, T, T, T>,
    // lcd: LiquidCrystal<'a, Parallel<T, T, T, T, T, T, T>, 20, 2, Async>,
}
pub struct Config<T: OutputPin, D: DelayNs> {
    pub rs: T,
    pub en1: T,
    pub en2: T,
    pub d1: T,
    pub d2: T,
    pub d3: T,
    pub d4: T,
    pub delay: D,
}
impl<T: OutputPin> LiquidCrystalScreen<T> {
    pub async fn new<D: DelayNs>(mut config: Config<T, D>) -> Self {
        let mut interface = Parallel::new(
            config.d1, config.d2, config.d3, config.d4, config.rs, config.en1, config.en2,
        );
        let layout: Layout<SCREEN_WIDTH, SCREEN_HEIGHT> = Layout {
            addrs: [0x80, 0xC0],
        };

        let lcd = LiquidCrystal::new(&mut interface, BusBits::Bus4Bits, layout);
        let mut lcd = lcd.asynch();
        lcd.begin(&mut config.delay).await;

        Self {
            current_screen: ScreenData::Message(ScreenMessage::Custom {
                title: "Initializing...".to_string(),
                content: "".to_string(),
            }),
            interface,
        }
    }
}

impl<T: OutputPin + Send> Screen for LiquidCrystalScreen<T> {
    async fn update_display(&mut self, screen_data: ScreenData) -> () {
        todo!();
    }

    fn get_current_screen(&self) -> ScreenData {
        self.current_screen.clone()
    }
}
