use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr};

use log::{debug, info};
use rand::seq::SliceRandom;
use tokio::net::UdpSocket;

use crate::common::{DNSServer, FlagRecordType, LabelSeq, ROOT_SERVERS};
use crate::config::Config;
use crate::message::Message;
use crate::question::Question;
use crate::resourserecord::{ResourceData, ResourceRecord};

pub async fn forward_recursive(question: Question, config: &Config) -> io::Result<Message> {
    send_question_to(question, config.forward_server_address_str()).await
}

async fn send_question_to(question: Question, server_addr: String) -> io::Result<Message> {
    let mut msg = Message::new();
    msg.add_question(question);
    send_message_to(msg, server_addr).await
}

async fn send_message_to(msg: Message, server_addr: String) -> io::Result<Message> {
    let raw_data = &msg.serialize()[..];

    info!("Forwarding to server at {server_addr}");

    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await.unwrap();
    let byte_sent = socket.send_to(raw_data, server_addr).await.unwrap();
    info!("Sent {byte_sent} bytes");

    let mut buff = vec![0; 1024];
    let msg_size = socket.recv(&mut buff).await.unwrap();
    info!("Received {msg_size} bytes");

    let reply = Message::parse(buff).unwrap();
    // info!("Get reply {:?}", reply);

    Ok(reply)
}

pub struct ForwardContext {
    pub cache: HashMap<Question, Vec<DNSServer>>,
}

impl ForwardContext {
    // TODO need this?
    pub fn new() -> ForwardContext {
        let root_domain_question = Question::new(LabelSeq::new(), FlagRecordType::A);
        let servers = ROOT_SERVERS.to_owned();
        ForwardContext {
            cache: HashMap::from([(root_domain_question, servers)]),
        }
    }
}

const MAX_ITER_FORWARD: usize = 10;

pub async fn forward_iterative(
    question: Question,
    config: &Config,
    context: &mut ForwardContext,
) -> io::Result<Message> {
    // TODO check cache before request
    // TODO select another server in case of failure
    let server_ref = ROOT_SERVERS
        .choose(&mut rand::thread_rng())
        .expect("cannot select random server");
    debug!("Use root server {:?}", server_ref);
    let mut server_addr = server_ref.to_addr_str();
    let mut counter = 0;

    loop {
        let res = send_question_to(question.clone(), server_addr)
            .await
            .expect("request to server failed");
        let ans = extract_answer(&res, &question.name);
        if ans.resources.len() > 0 {
            return Ok(res);
        }

        // TODO handle case where upstream dns server don't provide glue records
        let server_ref = ans
            .servers
            .choose(&mut rand::thread_rng())
            .expect("cannot select random server");
        debug!("Select server {:?}", server_ref);
        server_addr = server_ref.to_addr_str();

        if server_addr.is_empty() {
            debug!("No glue record found for server {:?}", server_ref);
            let question = Question::new(server_ref.name.clone(), FlagRecordType::A);
            let mut context = ForwardContext::new();
            let ans = Box::pin(forward_iterative(question, config, &mut context)).await?;
            let ip = extract_ip_v4(ans)[0];
            server_addr = format!("{ip}:53");
        }

        counter += 1;
        if counter > MAX_ITER_FORWARD {
            // TODO
            panic!("max iterative reached")
        }
    }
}

#[derive(Debug, Clone)]
struct Answer {
    servers: Vec<DNSServer>,
    resources: Vec<ResourceRecord>,
}

fn extract_answer(msg: &Message, requested_name: &LabelSeq) -> Answer {
    /**
    Note that DNS server may not provide glue records for all NS entries
    The NS entries are often randomly shuffled and the first ones usually have glue record
    Therefore, we should keep the order of NS record in the answer
    */

    let mut servers = vec![];
    let mut name_to_svrs = HashMap::new();
    let mut resources = vec![];

    let records = msg.resources.iter().chain(msg.auth_resources.iter());

    for record in records {
        match &record.data {
            ResourceData::NS(server_name) => {
                let server = DNSServer {
                    name: server_name.clone(),
                    ipv4addr: None,
                    ipv6addr: None,
                    port: 53,
                };
                servers.push(server);
                name_to_svrs.insert(server_name.clone(), servers.len() - 1);
            }
            _ => {
                if record.name == *requested_name {
                    resources.push(record.clone());
                } else {
                    debug!("Ignore resource: {:?}", record)
                }
            }
        }
    }

    for record in msg.addi_resources.iter() {
        if let Some(srv_idx) = name_to_svrs.get_mut(&record.name) {
            if let ResourceData::A(ip) = record.data {
                servers[*srv_idx].ipv4addr = Some(ip);
            }
            if let ResourceData::AAAA(ip) = record.data {
                servers[*srv_idx].ipv6addr = Some(ip);
            }
        }
    }

    Answer { servers, resources }
}

fn extract_ip_v4(msg: Message) -> Vec<Ipv4Addr> {
    msg.resources
        .iter()
        .chain(msg.auth_resources.iter())
        .filter_map(|r| {
            if let ResourceData::A(ip) = r.data {
                Some(ip)
            } else {
                None
            }
        })
        .collect()
}
