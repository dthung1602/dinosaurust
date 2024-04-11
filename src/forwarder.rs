use std::io;
use std::net::Ipv4Addr;

use log::info;
use tokio::net::UdpSocket;

use crate::config::Config;
use crate::message::Message;
use crate::question::Question;

pub async fn forward_recursive(question: Question, config: &Config) -> io::Result<Message> {
    let mut msg = Message::new();
    msg.add_question(question);

    let raw_data = &msg.serialize()[..];

    let upstream_server = format!(
        "{}:{}",
        config.forward_server_ip, config.forward_server_port
    );
    info!("Forwarding to server at {upstream_server}");

    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await.unwrap();
    let byte_sent = socket.send_to(raw_data, upstream_server).await.unwrap();
    info!("Sent {byte_sent} bytes");

    let mut buff = vec![0; 1024];
    let msg_size = socket.recv(&mut buff).await.unwrap();
    info!("Received {msg_size} bytes");

    let reply = Message::parse(buff).unwrap();
    info!("Get reply {:?}", reply);

    Ok(reply)
}
