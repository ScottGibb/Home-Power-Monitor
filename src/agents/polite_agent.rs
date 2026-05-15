use post_haste::agent::{Agent, Inbox};
use tokio::time::Duration;
use tokio::time::sleep;

use crate::{Addresses, Payloads, postmaster};

pub(crate) struct PoliteAgent {
    address: Addresses,
}

impl Agent for PoliteAgent {
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
                Payloads::Hello => self.handle_hello(received_message.source).await,
                _ => println!(
                    "PoliteAgent received the following message: {:?}",
                    received_message.payload
                ),
            };
        }
    }
}

impl PoliteAgent {
    async fn handle_hello(&self, source: Addresses) {
        println!("{:?} got hello from {:?}!", self.address, source);
        sleep(Duration::from_secs(1)).await;
        postmaster::send(source, self.address, Payloads::Hello)
            .await
            .unwrap();
    }
}
