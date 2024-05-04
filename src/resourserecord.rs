use std::net::{Ipv4Addr, Ipv6Addr};

use crate::common::{FlagClassCode, FlagRecordType, LabelSeq, ParseContext, SerializeContext};
use crate::utils::to_array;

#[derive(Debug, Clone)]
pub struct SOARecord {
    mname: LabelSeq,
    rname: LabelSeq,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    minimum: u32,
}

const SOA_FIXED_SIZE: usize = 20;

impl SOARecord {
    pub fn serialize(&self, context: &mut SerializeContext) {
        self.mname.serialize(context);
        self.rname.serialize(context);

        let bytes = self.serial.to_be_bytes();
        context.extend_from_slice(&bytes);

        let bytes = self.refresh.to_be_bytes();
        context.extend_from_slice(&bytes);

        let bytes = self.retry.to_be_bytes();
        context.extend_from_slice(&bytes);

        let bytes = self.expire.to_be_bytes();
        context.extend_from_slice(&bytes);

        let bytes = self.minimum.to_be_bytes();
        context.extend_from_slice(&bytes);
    }

    pub fn parse(context: &mut ParseContext) -> Result<SOARecord, *const str> {
        let mname = LabelSeq::parse(context)?;
        let rname = LabelSeq::parse(context)?;

        let slice = context.current_slice();
        if slice.len() < SOA_FIXED_SIZE {
            return Err("cannot parse SOA record");
        }

        let record = SOARecord {
            mname,
            rname,
            serial: u32::from_be_bytes(to_array(&slice[0..4])),
            refresh: u32::from_be_bytes(to_array(&slice[4..8])),
            retry: u32::from_be_bytes(to_array(&slice[8..12])),
            expire: u32::from_be_bytes(to_array(&slice[12..16])),
            minimum: u32::from_be_bytes(to_array(&slice[16..20])),
        };
        context.advance(SOA_FIXED_SIZE);

        Ok(record)
    }
}

#[derive(Debug, Clone)]
pub enum ResourceData {
    A(Ipv4Addr),
    NS(LabelSeq),
    CNAME(LabelSeq),
    AAAA(Ipv6Addr),
    SOA(SOARecord),
}

#[derive(Debug, Clone)]
pub struct ResourceRecord {
    pub name: LabelSeq,
    pub record_type: FlagRecordType,
    pub class_code: FlagClassCode,
    pub ttl: u32,
    pub length: u16,
    pub data: ResourceData,
}

impl ResourceRecord {
    pub fn serialize(&self, context: &mut SerializeContext) {
        self.name.serialize(context);

        let record_type = self.record_type.bits();
        let class_code = self.class_code.bits();

        let mut rest = vec![
            (record_type >> 8) as u8,
            record_type as u8,
            (class_code >> 8) as u8,
            class_code as u8,
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
            ResourceData::SOA(soa) => soa.serialize(context),
            ResourceData::NS(seq) | ResourceData::CNAME(seq) => seq.serialize(context),
        }
    }

    pub fn parse(context: &mut ParseContext) -> Result<ResourceRecord, *const str> {
        let name = LabelSeq::parse(context)?;

        let data = context.current_slice();
        if data.len() < 10 {
            return Err("cannot parse record");
        }

        let flag = u16::from_be_bytes([data[0], data[1]]);
        let record_type: FlagRecordType =
            FlagRecordType::from_bits(flag).expect("cannot parse record type");

        let flag = u16::from_be_bytes([data[2], data[3]]);
        let class_code: FlagClassCode =
            FlagClassCode::from_bits(flag).expect("cannot parse class code");

        let ttl = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let length = u16::from_be_bytes([data[8], data[9]]);

        context.advance(10);
        let data = Self::parse_data(context, record_type.clone(), length as usize)?;

        Ok(ResourceRecord {
            name,
            record_type,
            class_code,
            ttl,
            length,
            data,
        })
    }

    fn parse_data(
        context: &mut ParseContext,
        record_type: FlagRecordType,
        size: usize,
    ) -> Result<ResourceData, *const str> {
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
                let ip = Ipv6Addr::from(to_array(&buff[0..16]));
                context.advance(16);
                ResourceData::AAAA(ip)
            }
            FlagRecordType::SOA => {
                let soa = SOARecord::parse(context)?;
                if context.current_idx() != max_index {
                    return Err("sequence in record exceed specified length");
                }
                ResourceData::SOA(soa)
            }
            FlagRecordType::NS => {
                let seq = LabelSeq::parse(context)?;
                if context.current_idx() != max_index {
                    return Err("sequence in record exceed specified length");
                }
                ResourceData::NS(seq)
            }
            FlagRecordType::CNAME => {
                let seq = LabelSeq::parse(context)?;
                if context.current_idx() != max_index {
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
