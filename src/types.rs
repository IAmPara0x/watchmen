use surge_ping::{Client, PingIdentifier, PingSequence, Pinger, SurgeError};
use crate::{config::WatchmenConfig, notify::Notify};
use std::{thread, time::{Duration}};

#[derive(PartialEq)]
pub enum Status {
    Online,
    Offline
}

pub struct WatchmenState<T: Notify> {
    pub pinger: Pinger,
    pub server_status: Status,
    pub interval: Duration,
    pub services: Vec<T>,
    pub num_retries: u64,
    pub max_retries: u64
}


impl <T: Notify> WatchmenState<T> {

    pub async fn new(client: &Client, config: &WatchmenConfig ) -> Option<WatchmenState<T>> {

        let pinger = client.pinger(config.server_addr.parse().ok()?, PingIdentifier(1)).await;
        let state = WatchmenState {pinger, server_status: Status::Offline, interval: Duration::from_secs(config.poll_interval), services: vec![], max_retries: config.max_retries, num_retries: 0 };
        Some(state)
    }

    async fn all_services_notify(&self, msg: &String) {
        for service in self.services.iter() {
            service.notify(msg).await; 
        }
    }

    async fn handle_online(&mut self) {

        println!("Handling online...");
        if self.server_status == Status::Offline {
            self.all_services_notify(&"Server is back online again!".to_string()).await;
            self.server_status = Status::Online;
            self.num_retries = 0;
        }
    }

    async fn handle_offline(&mut self) {

        println!("Handling offline...");

        if self.server_status == Status::Online {
            if self.num_retries == self.max_retries {
                self.all_services_notify(&"Server is offline!".to_string()).await;
                self.server_status = Status::Offline;
            } else {

                println!("Increasing the retry count to: {:?}", self.num_retries);
                self.num_retries += 1
            }
        }

    }

    pub async fn watch(&mut self) -> () {

        let payload = [0; 8];
        loop {

            println!("pinging: {:?}", self.pinger.host);
            match self.pinger.ping(PingSequence(1), &payload).await  {
                Ok((_packet, _duration)) => self.handle_online().await,
                Err(SurgeError::Timeout{..}) => self.handle_offline().await,
                Err(err) => {
                    self.all_services_notify(&"Ping to the server failed.".to_string()).await;
                    eprintln!("ERROR: {:?}", err);
                },
            }
            println!("sleeping...");
            thread::sleep(self.interval);
        }

    }

}
