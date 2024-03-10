mod config;

use tokio;
use tokio::net::{UdpSocket};
use std::io;

const DINOSAURUST: &str = "
    ____  _                                              __
   / __ \\(_)___  ____  _________ ___  _________  _______/ /_
  / / / / / __ \\/ __ \\/ ___/ __ `/ / / / ___/ / / / ___/ __/
 / /_/ / / / / / /_/ (__  ) /_/ / /_/ / /  / /_/ (__  ) /_
/_____/_/_/ /_/\\____/____/\\__,_/\\__,_/_/   \\__,_/____/\\__/
";

#[tokio::main]
async fn main() -> io::Result<()> {
    let cfg = config::load_config();
    println!("{}", DINOSAURUST);

    let addr = format!("{}:{}", cfg.ip, cfg.port);
    println!("Start listening at {}", addr);
    let sock = UdpSocket::bind(addr).await?;

    let mut buff = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buff).await?;
        let x = &buff[..len];
        let content = String::from_utf8(Vec::from(x)).unwrap();
        println!("Get message: {content}");
        sock.send_to(x, addr).await?;
    }

    Ok(())
}
