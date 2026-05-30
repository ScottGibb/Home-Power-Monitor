//! The "button" task, which asynchronously awaits for the user to press enter,
//! signalling that the pedestrian wants to cross

use std::collections::HashMap;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use tracing::{error, info, trace};

use crate::{
    agents::{Addresses, Payloads, inputs::Button},
    postmaster,
};

pub type TerminalButtonConfig = HashMap<String, Button>;
pub struct TerminalCommandAgent {
    pub keymap: TerminalButtonConfig,
    pub receivers: Vec<Addresses>,
}

impl TerminalCommandAgent {
    pub async fn button_task(self) -> ! {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        loop {
            if let Ok(Some(line)) = reader.next_line().await {
                if let Some(button) = self.keymap.get(&line) {
                    trace!(key = %line, "ButtonAgent detected a button press");
                    for receiver in &self.receivers {
                        match postmaster::send(
                            *receiver,
                            Addresses::Button,
                            Payloads::ButtonPressed(*button),
                        )
                        .await
                        {
                            Ok(_) => (),
                            Err(e) => error!(error = ?e, "ButtonAgent failed to send message"),
                        }
                    }
                }
            }
        }
    }
}
