use jsy_mk_194_rs::units::{kilowatt, kilowatt_hour, watt};
use tracing::info;

use crate::agents::screen::{
    screen::{Screen, ScreenData, ScreenMessage},
    utils::{center_string, string_to_char_array},
};
use std::io::{self, Write};

const SCREEN_WIDTH: usize = 20;
const SCREEN_HEIGHT: usize = 2;
pub struct MockScreen {
    current_screen: ScreenData,
    screen_buffer: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
    terminal_initialized: bool,
}

impl MockScreen {
    pub async fn new() -> Self {
        let mut screen = Self {
            current_screen: ScreenData::Message(ScreenMessage::Error("No data yet".to_string())),
            screen_buffer: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            terminal_initialized: false,
        };
        screen
            .update_display(ScreenData::Message(ScreenMessage::Error(
                "No data yet".to_string(),
            )))
            .await;
        screen
    }
}

impl Drop for MockScreen {
    fn drop(&mut self) {
        // Restore cursor and leave alternate screen buffer when the app exits.
        print!("\x1b[?25h\x1b[?1049l");
        let _ = io::stdout().flush();
    }
}

impl Screen for MockScreen {
    async fn update_display(&mut self, screen_data: ScreenData) -> () {
        let screen_string = self.calculate_screen_string(&screen_data);

        self.write_to_screen_buffer(&screen_string);
        self.current_screen = screen_data;
    }

    fn get_current_screen(&self) -> ScreenData {
        self.current_screen.clone()
    }
}

impl MockScreen {
    fn calculate_screen_string(
        &self,
        screen_data: &ScreenData,
    ) -> [char; SCREEN_WIDTH * SCREEN_HEIGHT] {
        let string = match screen_data {
            ScreenData::Average(avg_power) => {
                let avg_power = avg_power.get::<watt>();
                let title = match center_string("Average Power (W)", SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Average");
                        format!("Err: {}", err)
                    }
                };
                let value = match center_string(&format!("{:.2}", avg_power), SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Value too wide for Average");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, value)
            }

            ScreenData::Instantaneous(instant_power) => {
                let instant_power = instant_power.get::<watt>();
                let title = match center_string("Instant Power (W)", SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Instantaneous");
                        format!("Err: {}", err)
                    }
                };
                let value = match center_string(&format!("{}", instant_power), SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Value too wide for Instantaneous");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, value)
            }
            ScreenData::Daily {
                current_power,
                energy,
            } => {
                let current_power = current_power.get::<kilowatt>();
                let energy = energy.get::<kilowatt_hour>();
                let title = match center_string("Daily Usage (kW/kWh)", SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Daily");
                        format!("Err: {}", err)
                    }
                };
                let value = match center_string(
                    &format!("P:{:.1} E:{:.1}", current_power, energy),
                    SCREEN_WIDTH,
                ) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Value too wide for Daily");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, value)
            }
            ScreenData::Monthly {
                total_energy,
                daily_low: _,
                daily_avg,
                daily_high: _,
            } => {
                let total_energy = total_energy.get::<kilowatt_hour>();
                let daily_avg = daily_avg.get::<kilowatt_hour>();

                let title = match center_string("Monthly Usage (kWh)", SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Monthly");
                        format!("Err: {}", err)
                    }
                };
                let value = match center_string(
                    &format!("T:{:.1} A:{:.1}", total_energy, daily_avg),
                    SCREEN_WIDTH,
                ) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Value too wide for Monthly");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, value)
            }
            ScreenData::Yearly {
                lowest_day: _,
                avg_day: _,
                highest_day: _,
                total_energy,
            } => {
                let total_energy = total_energy.get::<kilowatt_hour>();
                let title = match center_string("Yearly Usage (kWh)", SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Yearly");
                        format!("Err: {}", err)
                    }
                };
                let value = match center_string(&format!("{:.3}", total_energy), SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Value too wide for Yearly");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, value)
            }
            ScreenData::Message(screen_message) => {
                let (title, content) = match screen_message {
                    ScreenMessage::Custom { title, content } => (title, content),
                    ScreenMessage::Error(err) => (&"Error".to_owned(), err),
                };
                let title = match center_string(title, SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Title too wide for Message");
                        format!("Err: {}", err)
                    }
                };
                let content = match center_string(content, SCREEN_WIDTH) {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::error!(%err, "Content too wide for Message");
                        format!("Err: {}", err)
                    }
                };
                format!("{}\n{}", title, content)
            }
        };
        string_to_char_array::<{ SCREEN_WIDTH * SCREEN_HEIGHT }>(&string)
    }
    fn write_to_screen_buffer(&mut self, char_array: &[char; SCREEN_WIDTH * SCREEN_HEIGHT]) {
        // Convert the char array back to a string for logging
        let flat_string: String = char_array.iter().collect();

        // Split the flat string into lines by '\n', pad/truncate each line to SCREEN_WIDTH
        let lines: Vec<&str> = flat_string.split('\n').collect();
        for row in 0..SCREEN_HEIGHT {
            let line = lines.get(row).copied().unwrap_or("");
            let mut chars: Vec<char> = line.chars().collect();
            // Pad or truncate to SCREEN_WIDTH
            if chars.len() < SCREEN_WIDTH {
                chars.extend(std::iter::repeat(' ').take(SCREEN_WIDTH - chars.len()));
            } else if chars.len() > SCREEN_WIDTH {
                chars.truncate(SCREEN_WIDTH);
            }
            for col in 0..SCREEN_WIDTH {
                self.screen_buffer[row][col] = chars[col];
            }
        }

        if !self.terminal_initialized {
            // Enter alternate screen and hide cursor for a clean app-owned surface.
            print!("\x1b[?1049h\x1b[2J\x1b[H\x1b[?25l");
            self.terminal_initialized = true;
        }

        // Redraw entire app surface each frame.
        print!("\x1b[2J\x1b[H");
        for _ in 0..(SCREEN_WIDTH + 2) {
            print!("-");
        }
        println!();
        for row in 0..SCREEN_HEIGHT {
            let row_string: String = self.screen_buffer[row].iter().collect();
            println!("|{}|", row_string);
        }
        for _ in 0..(SCREEN_WIDTH + 2) {
            print!("-");
        }
        println!();

        let _ = io::stdout().flush();
    }
}
