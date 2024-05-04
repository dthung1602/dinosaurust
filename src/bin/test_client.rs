use std::io;
use std::net::Ipv4Addr;

use env_logger::Env;
use log::info;
use tokio::net::UdpSocket;

use dinosaurust::common::{FlagRD, FlagRecordType};
use dinosaurust::message::Message;
use dinosaurust::question::Question;

const DINOSAURUST_ADDRESS: &str = "127.0.0.1:2053";

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut msg = Message::new();
    msg.header.set_rd(FlagRD::TRUE);

    let q1 = Question::new("www.google.com".into(), FlagRecordType::AAAA);
    msg.add_question(q1);
    // let q2 = Question::new("google.com".to_string(), FlagRecordType::A);
    // msg.add_question(q2);
    let msg_data = &msg.serialize()[..];

    info!("Sending request to {DINOSAURUST_ADDRESS}");
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await.unwrap();
    socket
        .send_to(&msg_data, DINOSAURUST_ADDRESS)
        .await
        .unwrap();

    let mut buff = vec![0; 1024];
    let msg_size = socket.recv(&mut buff).await.unwrap();
    info!("Received {msg_size} bytes");

    let reply = Message::parse(buff).unwrap();
    info!("Get reply {:?}", reply);

    Ok(())
}
