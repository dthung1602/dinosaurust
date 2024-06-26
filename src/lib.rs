#![allow(dead_code)]

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use log::{debug, info};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::forwarder::ForwardContext;
use config::Config;

use crate::message::Message;

pub mod common;
pub mod config;
pub mod forwarder;
pub mod header;
pub mod message;
pub mod question;
pub mod resourserecord;
mod utils;

pub struct DinosaurustServer {
    cfg: Config,
    stoptx: Option<mpsc::Sender<()>>,
}

type ResponsePair = (Vec<u8>, SocketAddr);

impl DinosaurustServer {
    pub fn new() -> DinosaurustServer {
        DinosaurustServer {
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
                // for x in &buff {
                //     debug!("{:08b} ", x)
                // }
                // debug!("\n\n--------\n\n");
            }
        });

        // Channel to propagate stop signal
        let (stoptx, mut stoprx) = mpsc::channel(1);
        self.stoptx = Some(stoptx);

        // Task to accept UDP datagram
        let cfg = self.cfg.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(_) = stoprx.try_recv() {
                    break;
                }
                let mut buff = vec![0; 1024];
                let (_len, peer_addr) = sock.recv_from(&mut buff).await.unwrap();
                let tx_clone = tx.clone();
                let cfg = cfg.clone();
                tokio::spawn(async move { handle_request(cfg, buff, tx_clone, peer_addr).await });
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

async fn handle_request(
    cfg: Config,
    buff: Vec<u8>,
    tx: mpsc::Sender<ResponsePair>,
    addr: SocketAddr,
) {
    // TODO handle & response error
    let request = Message::parse(buff).unwrap();

    debug!("\nGet request: {:?}", request);
    // debug!("Flags: {}", request.header.flags);
    // debug!("QR {:?}", request.header.get_qr());
    // debug!("OPCODE {:?}", request.header.get_opcode());
    // debug!("AA {:?}", request.header.get_aa());
    // debug!("TC {:?}", request.header.get_tc());
    // debug!("RD {:?}", request.header.get_rd());
    // debug!("RA {:?}", request.header.get_ra());
    // debug!("RC {:?}", request.header.get_rcode());

    let mut context = ForwardContext::new();
    let res = forwarder::forward_iterative(request.questions[0].clone(), &cfg, &mut context)
        .await
        .unwrap();

    let mut reply = Message::reply_to(&request);
    reply.copy_resources(&res);

    debug!("\nReply: {:?}", reply);

    let res = reply.serialize();

    tx.send((res, addr)).await.unwrap()
}
