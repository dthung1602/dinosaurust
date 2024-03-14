use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use common::RecordType;
use config::Config;
use header::Header;
use question::Question;
use resourserecord::ResourceRecord;

mod common;
mod config;
mod header;
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
                println!("Sent {len} bytes to {addr}:");
                for x in &buff {
                    print!("{:08b} ", x)
                }
                println!();
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
    let content = String::from_utf8(buff).unwrap();
    println!("Get message: {content}");

    let mut header = Header::new_reply();
    let question = Question::new("www.google.com".to_string(), RecordType::A);
    let record = ResourceRecord::new("www.google.com".to_string());
    header.n_question = 1;
    header.n_answer = 1;

    let mut res = header.to_vec();
    let mut question_data = question.to_vec();
    let mut record_data = record.to_vec();
    res.append(&mut question_data);
    res.append(&mut record_data);

    tx.send((res, addr)).await.unwrap()
}
