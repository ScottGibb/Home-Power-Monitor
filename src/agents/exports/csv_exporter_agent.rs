use std::path::PathBuf;

use post_haste::agent::{Agent, Inbox};

use crate::{
    agents::{Addresses, Payloads, power_meter_agent::PowerReading},
    postmaster,
};
use csv_async::AsyncWriter;
use jsy_mk_194_rs::units::{ampere, volt, watt};
use tokio::fs::{File, OpenOptions};

pub struct CSVExporterAgent {
    _address: Addresses,
    writer: AsyncWriter<File>,
}

pub struct Config {
    pub file_path: PathBuf,
}

impl Agent for CSVExporterAgent {
    type Address = Addresses;
    type Message = postmaster::Message;
    type Config = Config;

    async fn create(address: Self::Address, config: Self::Config) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.file_path)
            .await
            .expect("Failed to open CSV file");

        let metadata = tokio::fs::metadata(&config.file_path)
            .await
            .expect("Failed to get file metadata");

        let mut writer = AsyncWriter::from_writer(file);

        // Write headers if file is empty
        if metadata.len() == 0 {
            let headers = vec![
                "timestamp".to_string(),
                "voltage".to_string(),
                "current".to_string(),
                "active_power".to_string(),
                "power_factor".to_string(),
                "power_direction".to_string(),
            ];
            writer
                .write_record(&headers)
                .await
                .expect("Failed to write CSV headers");
        }

        Self {
            _address: address,
            writer,
        }
    }

    async fn run(mut self, mut inbox: Inbox<Self::Message>) -> ! {
        loop {
            let received_message = inbox.recv().await.unwrap();
            match &received_message.payload {
                Payloads::PowerReading(reading) => {
                    // Append the reading to the CSV file
                    match self.append_to_csv(reading).await {
                        Ok(_) => {
                            (println!("CSVExporterAgent successfully wrote a new record to CSV"))
                        }
                        Err(e) => eprintln!("Failed to write to CSV: {:?}", e),
                    }
                }
                _ => {
                    eprintln!(
                        "CSVExporterAgent received unexpected message: {:?}",
                        received_message.payload
                    );
                }
            }
        }
    }
}

impl CSVExporterAgent {
    async fn append_to_csv(&mut self, reading: &PowerReading) -> tokio::io::Result<()> {
        let record = vec![
            reading.timestamp.to_rfc3339(),
            reading.reading.voltage.get::<volt>().to_string(),
            reading.reading.current.get::<ampere>().to_string(),
            reading.reading.active_power.get::<watt>().to_string(),
            reading.reading.power_factor.to_string(),
            reading.reading.power_direction.as_ref().to_string(),
        ];
        self.writer.write_record(&record).await?;
        self.writer.flush().await?;

        Ok(())
    }
}
