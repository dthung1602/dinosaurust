use crate::common::{FlagClassCode, FlagRecordType, LabelSeq};

#[derive(Debug, Clone)]
pub struct Question {
    name: LabelSeq,
    record_type: u16,
    class_code: u16,
}

impl Question {
    pub fn new(raw_name: String, record_type: FlagRecordType) -> Question {
        Question {
            name: LabelSeq::from_string(raw_name).unwrap(),
            record_type: record_type.bits(),
            class_code: FlagClassCode::IN.bits(),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = self.name.to_vec();
        res.push(0);
        res.push((self.record_type >> 8) as u8);
        res.push(self.record_type as u8);
        res.push((self.class_code >> 8) as u8);
        res.push(self.class_code as u8);
        res
    }

    // Return: (Question, next_index_of_buff_to_parse)
    pub fn parse(buff: &[u8]) -> Result<(Question, usize), *const str> {
        let (label_seq, next_idx) = LabelSeq::parse(buff)?;
        if next_idx + 3 > buff.len() - 1 {
            return Err("cannot read record type and class code");
        }

        let record_type = u16::from_be_bytes([buff[next_idx], buff[next_idx + 1]]);
        let class_code = u16::from_be_bytes([buff[next_idx + 2], buff[next_idx + 3]]);
        let question = Question {
            name: label_seq,
            record_type,
            class_code,
        };

        Ok((question, next_idx + 4))
    }
}
