use post_haste::agent::{Agent, Inbox};
use tokio::time::Duration;
use tokio::time::sleep;

use crate::{Addresses, Payloads, postmaster};

pub(crate) struct DebugAgent {
    address: Addresses,
}

impl Agent for DebugAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = ();

    async fn create(address: Self::Address, _config: Self::Config) -> Self {
        Self { address }
    }

    async fn run(self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            match &received_message.payload {
                _ => println!(
                    "DebugAgent received the following message: {:?}",
                    received_message.payload
                ),
            };
        }
    }
}
