#![allow(dead_code)]

use log::{debug, info};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use config::Config;
use resourserecord::ResourceRecord;

use crate::message::DNSMessage;

pub mod common;
pub mod config;
pub mod header;
pub mod message;
pub mod question;
pub mod resourserecord;

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
        info!("Started listening at {}", addr);

        let sock = Arc::new(UdpSocket::bind(addr).await?);
        let (tx, mut rx) = mpsc::channel::<ResponsePair>(1024);

        // Task to send response
        let sock_clone = sock.clone();
        tokio::spawn(async move {
            while let Some((buff, addr)) = rx.recv().await {
                let len = sock_clone.send_to(&buff[..], addr).await.unwrap();
                debug!("\nSent {len} bytes to {addr}:");
                for x in &buff {
                    debug!("{:08b} ", x)
                }
                debug!("\n\n--------\n\n");
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
            info!("Stopping server");
            tx.send(()).await.unwrap()
        }
    }
}

async fn handle_request(buff: Vec<u8>, tx: mpsc::Sender<ResponsePair>, addr: SocketAddr) {
    let request = DNSMessage::parse(buff).unwrap();

    debug!("\nGet request: {:?}", request);
    debug!("Flags: {}", request.header.flags);
    debug!("QR {:?}", request.header.get_qr());
    debug!("OPCODE {:?}", request.header.get_opcode());
    debug!("AA {:?}", request.header.get_aa());
    debug!("TC {:?}", request.header.get_tc());
    debug!("RD {:?}", request.header.get_rd());
    debug!("RA {:?}", request.header.get_ra());
    debug!("RC {:?}", request.header.get_rcode());

    let mut reply = DNSMessage::reply_to(&request);
    let record = ResourceRecord::new("www.google.com".to_string());
    reply.add_resource(record);

    debug!("\nReply: {:?}", reply);

    let res = reply.to_vec();

    tx.send((res, addr)).await.unwrap()
}
