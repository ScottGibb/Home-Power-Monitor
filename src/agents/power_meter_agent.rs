use post_haste::agent::{Agent, Inbox};

use crate::{
    agents::{Addresses, Payloads},
    postmaster,
};

pub struct PowerMeterAgent {
    _address: Addresses,
}

impl Agent for PowerMeterAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = ();

    async fn create(_address: Self::Address, _config: Self::Config) -> Self {
        Self { _address }
    }

    async fn run(self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            match &received_message.payload {
                _ => println!(
                    "PowerMeterAgent received an unknown message: {:?}",
                    received_message.payload
                ),
            };
        }
    }
}
