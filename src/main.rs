use std::io;

use tokio;
use tokio::net::UdpSocket;

use crate::common::RecordType;
use crate::header::Header;
use crate::question::Question;
use crate::resourserecord::ResourceRecord;

mod config;
mod common;
mod header;
mod question;
mod resourserecord;

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

        for x in &res {
            print!("{:08b} ", x)
        }
        println!();

        sock.send_to(&res[..], addr).await?;
    }

    Ok(())
}
