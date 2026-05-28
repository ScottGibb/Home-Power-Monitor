use crate::agents::screen::screen::{Screen, ScreenData};
use std::io::{self, Write};

const SCREEN_WIDTH: usize = 20;
const SCREEN_HEIGHT: usize = 2;
pub struct MockScreen {
    pub current_screen: ScreenData,
    screen_buffer: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
    terminal_initialized: bool,
}

impl Default for MockScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl MockScreen {
    pub fn new() -> Self {
        Self {
            current_screen: ScreenData::Error("No data yet".to_string()),
            screen_buffer: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            terminal_initialized: false,
        }
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
        if self.current_screen == screen_data {
            return;
        }
        self.current_screen = screen_data;
        let screen_string = self.current_screen.to_string();
        let mut chars = screen_string.chars();
        for row in 0..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                self.screen_buffer[row][col] = chars.next().unwrap_or(' ');
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

    fn get_current_screen(&self) -> ScreenData {
        self.current_screen.clone()
    }
}
