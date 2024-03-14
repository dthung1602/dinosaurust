use tokio;
use tokio::signal;

use dinosaurust::DNSServer;

const DINOSAURUST: &str = "
    ____  _                                              __
   / __ \\(_)___  ____  _________ ___  _________  _______/ /_
  / / / / / __ \\/ __ \\/ ___/ __ `/ / / / ___/ / / / ___/ __/
 / /_/ / / / / / /_/ (__  ) /_/ / /_/ / /  / /_/ (__  ) /_
/_____/_/_/ /_/\\____/____/\\__,_/\\__,_/_/   \\__,_/____/\\__/
";

#[tokio::main]
async fn main() {
    println!("{}", DINOSAURUST);

    let mut server = DNSServer::new();
    server.start().await.unwrap();

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Get signal Ctrl+C");
            server.stop().await;
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
}
