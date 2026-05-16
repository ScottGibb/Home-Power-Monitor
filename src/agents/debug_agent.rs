use post_haste::agent::{Agent, Inbox};

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
            println!(
                "DebugAgent received the following message: {:?}",
                received_message.payload
            );
        }
    }
}
