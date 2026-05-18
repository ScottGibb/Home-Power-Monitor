use post_haste::agent::{Agent, Inbox};
use tracing::info;

use crate::{Addresses, postmaster};

pub struct DebugAgent {
    _address: Addresses,
}

impl Agent for DebugAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = ();

    async fn create(_address: Self::Address, _config: Self::Config) -> Self {
        Self { _address }
    }

    async fn run(self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            info!(payload = ?received_message.payload, "DebugAgent received message");
        }
    }
}
