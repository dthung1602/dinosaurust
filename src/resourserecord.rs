use std::net::Ipv4Addr;

use bitflags::Flags;

use crate::common::{FlagClassCode, FlagRecordType};

#[derive(Debug)]
enum ResourceData {
    A(Ipv4Addr),
}

#[derive(Debug)]
pub struct ResourceRecord {
    name: Vec<String>,
    record_type: u16,
    class_code: u16,
    ttl: u32,
    length: u16,
    data: ResourceData,
}

impl ResourceRecord {
    pub fn new(raw_name: String) -> ResourceRecord {
        let name: Vec<String> = raw_name.split('.').map(|s| s.to_string()).collect();
        let ip = Ipv4Addr::new(1, 1, 1, 1);
        ResourceRecord {
            name,
            record_type: FlagRecordType::A.bits(),
            class_code: FlagClassCode::IN.bits(),
            ttl: 111,
            length: 4,
            data: ResourceData::A(ip),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = vec![];
        for part in &self.name {
            let bytes = part.as_bytes();
            let len = bytes.len() as u8;
            res.push(len);
            res.extend_from_slice(bytes);
        }

        let mut rest = vec![
            0,
            (self.record_type >> 8) as u8,
            self.record_type as u8,
            (self.class_code >> 8) as u8,
            self.class_code as u8,
            (self.ttl >> 24) as u8,
            (self.ttl >> 16) as u8,
            (self.ttl >> 8) as u8,
            self.ttl as u8,
            (self.length >> 8) as u8,
            self.length as u8,
        ];
        res.append(&mut rest);

        match self.data {
            ResourceData::A(ip) => {
                let mut ip = ip.octets().to_vec();
                res.append(&mut ip)
            }
        }

        res
    }
}
