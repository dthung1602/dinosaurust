use env_logger;
use env_logger::Env;
use log::{error, warn};
use tokio;
use tokio::signal;

use dinosaurust::DinosaurustServer;

const DINOSAURUST: &str = "
    ____  _                                              __
   / __ \\(_)___  ____  _________ ___  _________  _______/ /_
  / / / / / __ \\/ __ \\/ ___/ __ `/ / / / ___/ / / / ___/ __/
 / /_/ / / / / / /_/ (__  ) /_/ / /_/ / /  / /_/ (__  ) /_
/_____/_/_/ /_/\\____/____/\\__,_/\\__,_/_/   \\__,_/____/\\__/
";

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("{}", DINOSAURUST);

    let mut server = DinosaurustServer::new();
    server.start().await.unwrap();

    match signal::ctrl_c().await {
        Ok(()) => {
            warn!("Get signal Ctrl+C");
            server.stop().await;
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }
}
