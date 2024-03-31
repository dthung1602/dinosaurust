use std::net::Ipv4Addr;

use crate::common::{FlagClassCode, FlagRecordType, LabelSeq, SerializeContext};

#[derive(Debug)]
enum ResourceData {
    A(Ipv4Addr),
}

#[derive(Debug)]
pub struct ResourceRecord {
    name: LabelSeq,
    record_type: u16,
    class_code: u16,
    ttl: u32,
    length: u16,
    data: ResourceData,
}

impl ResourceRecord {
    pub fn new(raw_name: String) -> ResourceRecord {
        let ip = Ipv4Addr::new(1, 1, 1, 1);
        ResourceRecord {
            name: LabelSeq::from_string(raw_name).unwrap(),
            record_type: FlagRecordType::A.bits(),
            class_code: FlagClassCode::IN.bits(),
            ttl: 111,
            length: 4,
            data: ResourceData::A(ip),
        }
    }

    pub fn serialize(&self, context: &mut SerializeContext){
        self.name.serialize(context);

        let mut rest = vec![
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
        context.append(&mut rest);

        match self.data {
            ResourceData::A(ip) => {
                let mut ip = ip.octets().to_vec();
                context.append(&mut ip)
            }
        }
    }
}
