use reqwest::{Client};

use crate::config::WatchmenConfig;

pub trait Notify {
    async fn notify(&self, msg: &String) -> ();
}

pub struct ConsoleNotifier;
impl Notify for ConsoleNotifier {
     async fn notify(&self, msg: &String) -> () {
        println!("{}", msg);
    }
}

pub struct TelegramNotifier {
    bot_token: String,
    chat_id: String,
    client: Client
}

impl TelegramNotifier {
    pub fn new(config: &WatchmenConfig) -> TelegramNotifier {
        TelegramNotifier {client: Client::new(), chat_id: config.chat_id.clone(), bot_token: config.bot_token.clone()}
    }
}
//
impl Notify for TelegramNotifier {

    async fn notify(&self, msg: &String) -> () {

        let res = self.client.post(format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token))
                             .form(&[("chat_id", &self.chat_id), ("text", msg)])
                             .send()
                             .await.unwrap();

        println!("Response: {:?}", res);

    }
}
