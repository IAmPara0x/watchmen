
use surge_ping::{Config};
use surge_ping;

mod types;
mod notify;
mod config;

use types::*;

use notify::TelegramNotifier;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cfg = config::WatchmenConfig::load_from_file("./watchconfig.toml".to_string());
    let surge_client = surge_ping::Client::new(&Config::new()).unwrap();

    let mut state: WatchmenState<TelegramNotifier> = WatchmenState::new(&surge_client, &cfg).await.unwrap();
    
    state.services.push(TelegramNotifier::new(&cfg));
    state.watch().await;

    Ok(())
}
