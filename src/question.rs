use crate::common::{FlagClassCode, FlagRecordType, LabelSeq, ParseContext, SerializeContext};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Question {
    pub name: LabelSeq,
    pub record_type: u16,
    pub class_code: u16,
}

impl Question {
    pub fn new(name: LabelSeq, record_type: FlagRecordType) -> Question {
        Question {
            name,
            record_type: record_type.bits(),
            class_code: FlagClassCode::IN.bits(),
        }
    }

    pub fn serialize(&self, context: &mut SerializeContext) {
        self.name.serialize(context);
        context.push((self.record_type >> 8) as u8);
        context.push(self.record_type as u8);
        context.push((self.class_code >> 8) as u8);
        context.push(self.class_code as u8);
    }

    pub fn parse(context: &mut ParseContext) -> Result<Question, *const str> {
        let label_seq = LabelSeq::parse(context)?;

        let buff = context.current_slice();
        if buff.len() < 4 {
            return Err("cannot read record type and class code");
        }

        let record_type = u16::from_be_bytes([buff[0], buff[1]]);
        let class_code = u16::from_be_bytes([buff[2], buff[3]]);
        let question = Question {
            name: label_seq,
            record_type,
            class_code,
        };

        context.advance(4);
        Ok(question)
    }
}
