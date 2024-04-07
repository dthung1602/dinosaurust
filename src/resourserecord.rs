use std::net::{Ipv4Addr, Ipv6Addr};

use crate::common::{FlagClassCode, FlagRecordType, LabelSeq, ParseContext, SerializeContext};
use crate::utils::{pop};

#[derive(Debug)]
enum ResourceData {
    A(Ipv4Addr),
    NS(LabelSeq),
    CNAME(LabelSeq),
    AAAA(Ipv6Addr),
}

#[derive(Debug)]
pub struct ResourceRecord {
    name: LabelSeq,
    // TODO store enum instead of int?
    record_type: u16,
    class_code: u16,
    ttl: u32,
    length: u16,
    data: ResourceData,
}

impl ResourceRecord {
    pub fn new() -> ResourceRecord {
        ResourceRecord {
            name: LabelSeq::new(),
            record_type: 0,
            class_code: 0,
            ttl: 0,
            length: 0,
            data: ResourceData::A(Ipv4Addr::new(0, 0, 0, 0)),
        }
    }

    pub fn serialize(&self, context: &mut SerializeContext) {
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

        match &self.data {
            ResourceData::A(ip) => {
                let mut ip = ip.octets().to_vec();
                context.append(&mut ip)
            }
            ResourceData::AAAA(ip) => {
                let mut ip = ip.octets().to_vec();
                context.append(&mut ip)
            }
            ResourceData::NS(seq) | ResourceData::CNAME(seq) => {
                seq.serialize(context)
            }
        }
    }

    pub fn parse(context: &mut ParseContext) -> Result<ResourceRecord, *const str> {
        let mut resource = Self::new();
        resource.name = LabelSeq::parse(context)?;

        let data = context.current_slice();
        if data.len() < 10 {
            return Err("cannot parse record");
        }

        let flag = u16::from_be_bytes([data[0], data[1]]);
        let record_type: FlagRecordType = FlagRecordType::from_bits(flag).expect("cannot parse record type");
        resource.record_type = record_type.bits();

        let flag = u16::from_be_bytes([data[2], data[3]]);
        let class_code: FlagClassCode = FlagClassCode::from_bits(flag).expect("cannot parse class code");
        resource.class_code = class_code.bits();

        resource.ttl = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        resource.length = u16::from_be_bytes([data[8], data[9]]);

        context.advance(10);
        resource.data = Self::parse_data(context, record_type, resource.length as usize)?;

        Ok(resource)
    }

    fn parse_data(context: &mut ParseContext, record_type: FlagRecordType, size: usize) -> Result<ResourceData, *const str> {
        let buff = context.current_slice();
        if buff.len() < size {
            return Err("cannot parse resource data");
        }

        let max_index = context.current_idx() + size;

        let res = match record_type {
            FlagRecordType::A => {
                if size != 4 {
                    return Err("ipv4 must be 4 bytes");
                }
                let ip = Ipv4Addr::from([buff[0], buff[1], buff[2], buff[3]]);
                context.advance(4);
                ResourceData::A(ip)
            }
            FlagRecordType::AAAA => {
                if size != 16 {
                    return Err("ipv6 must be 16 bytes");
                }
                let ip = Ipv6Addr::from(pop(&buff[0..16]));
                context.advance(16);
                ResourceData::AAAA(ip)
            }
            FlagRecordType::NS => {
                let seq = LabelSeq::parse(context)?;
                if context.current_idx() > max_index {
                    return Err("sequence in record exceed specified length");
                }
                ResourceData::NS(seq)
            }
            FlagRecordType::CNAME => {
                let seq = LabelSeq::parse(context)?;
                if context.current_idx() > max_index {
                    return Err("sequence in record exceed specified length");
                }
                ResourceData::CNAME(seq)
            }
            _ => {
                return Err("cannot parse resource data");
            }
        };

        Ok(res)
    }
}
