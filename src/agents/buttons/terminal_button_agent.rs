//! The "button" task, which asynchronously awaits for the user to press enter,
//! signalling that the pedestrian wants to cross

use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::{
    agents::{Addresses, Payloads, buttons::Button},
    postmaster,
};

pub struct TerminalButtonAgent {
    pub key: char,
    pub button: Button,
    pub receivers: Vec<Addresses>,
}

impl TerminalButtonAgent {
    pub async fn button_task(self) -> ! {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        loop {
            if let Ok(Some(line)) = reader.next_line().await {
                if line == self.key.to_string() {
                    println!("ButtonAgent detected a button press!");
                    for receiver in &self.receivers {
                        match postmaster::send(
                            *receiver,
                            Addresses::Button,
                            Payloads::ButtonPressed(self.button),
                        )
                        .await
                        {
                            Ok(_) => (),
                            Err(e) => eprintln!("Failed to send message: {:?}", e),
                        }
                    }
                }
            }
        }
    }
}
