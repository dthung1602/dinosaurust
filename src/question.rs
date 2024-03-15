use crate::common::{FlagClassCode, FlagRecordType};
use bitflags::Flags;

pub struct Question {
    name: Vec<String>,
    record_type: u16,
    class_code: u16,
}

impl Question {
    pub fn new(raw_name: String, record_type: FlagRecordType) -> Question {
        let name: Vec<String> = raw_name.split('.').map(|s| s.to_string()).collect();
        Question {
            name,
            record_type: record_type.bits(),
            class_code: FlagClassCode::IN.bits(),
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
        res.push(0);
        res.push((self.record_type >> 8) as u8);
        res.push(self.record_type as u8);
        res.push((self.class_code >> 8) as u8);
        res.push(self.class_code as u8);
        res
    }
}
