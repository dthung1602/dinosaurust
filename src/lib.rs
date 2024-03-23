use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use rand::prelude::*;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use config::Config;
use question::Question;
use resourserecord::ResourceRecord;

use crate::common::FlagRecordType;
use crate::message::DNSMessage;

mod common;
mod config;
mod header;
mod message;
mod question;
mod resourserecord;

pub struct DNSServer {
    cfg: Config,
    stoptx: Option<mpsc::Sender<()>>,
}

type ResponsePair = (Vec<u8>, SocketAddr);

impl DNSServer {
    pub fn new() -> DNSServer {
        DNSServer {
            cfg: config::load_config(),
            stoptx: None,
        }
    }

    pub async fn start(&mut self) -> io::Result<()> {
        let addr = self.cfg.socket_address_str();
        println!("Started listening at {}", addr);

        let sock = Arc::new(UdpSocket::bind(addr).await?);
        let (tx, mut rx) = mpsc::channel::<ResponsePair>(1024);

        // Task to send response
        let sock_clone = sock.clone();
        tokio::spawn(async move {
            while let Some((buff, addr)) = rx.recv().await {
                let len = sock_clone.send_to(&buff[..], addr).await.unwrap();
                println!("\nSent {len} bytes to {addr}:");
                for x in &buff {
                    print!("{:08b} ", x)
                }
                println!("\n\n--------\n\n");
            }
        });

        // Channel to propagate stop signal
        let (stoptx, mut stoprx) = mpsc::channel(1);
        self.stoptx = Some(stoptx);

        // Task to accept UDP datagram
        tokio::spawn(async move {
            loop {
                if let Ok(_) = stoprx.try_recv() {
                    break;
                }
                let mut buff = vec![0; 1024];
                let (_len, peer_addr) = sock.recv_from(&mut buff).await.unwrap();
                let tx_clone = tx.clone();
                tokio::spawn(async move { handle_request(buff, tx_clone, peer_addr).await });
            }
        });

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(tx) = &self.stoptx {
            println!("Stopping server");
            tx.send(()).await.unwrap()
        }
    }
}

async fn handle_request(buff: Vec<u8>, tx: mpsc::Sender<ResponsePair>, addr: SocketAddr) {
    let request = DNSMessage::parse(buff).unwrap();

    println!("\nGet request: {:?}", request);

    let mut reply = DNSMessage::reply_to(&request);
    let record = ResourceRecord::new("www.google.com".to_string());
    reply.add_resource(record);

    println!("\nReply: {:?}", reply);

    let res = reply.to_vec();

    tx.send((res, addr)).await.unwrap()
}
