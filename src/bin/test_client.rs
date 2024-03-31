use std::io;

use env_logger::Env;
use log::info;
use tokio::net::UdpSocket;

use dinosaurust::common::FlagRecordType;
use dinosaurust::message::DNSMessage;
use dinosaurust::question::Question;

const SENDING_SOCKET: &str = "127.0.0.1:11153";
const DINOSAURUST_ADDRESS: &str = "127.0.0.1:2053";

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut msg = DNSMessage::new();
    let q1 = Question::new("www.google.com".to_string(), FlagRecordType::A);
    msg.add_question(q1);
    let q2 = Question::new("google.com".to_string(), FlagRecordType::A);
    msg.add_question(q2);
    let msg_data = &msg.serialize()[..];

    info!("Sending request from {SENDING_SOCKET} to {DINOSAURUST_ADDRESS}");
    let socket = UdpSocket::bind(SENDING_SOCKET).await.unwrap();
    socket
        .send_to(&msg_data, DINOSAURUST_ADDRESS)
        .await
        .unwrap();

    let mut buff = vec![0; 1024];
    let msg_size = socket.recv(&mut buff).await.unwrap();
    info!("Received {msg_size} bytes");

    let reply = DNSMessage::parse(buff).unwrap();
    info!("Get reply {:?}", reply);

    Ok(())
}
