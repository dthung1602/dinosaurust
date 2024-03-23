use crate::header::Header;
use crate::question::Question;
use crate::resourserecord::ResourceRecord;

#[derive(Debug)]
pub struct DNSMessage {
    header: Header,
    questions: Vec<Question>,
    resources: Vec<ResourceRecord>,
}

impl DNSMessage {
    pub fn new() -> DNSMessage {
        DNSMessage {
            header: Header::new(),
            questions: vec![],
            resources: vec![],
        }
    }

    pub fn reply_to(request: &DNSMessage) -> DNSMessage {
        let mut header = Header::reply_to(&request.header);
        let questions = request.questions.clone();
        header.n_question = questions.len() as u16;

        DNSMessage {
            header,
            questions,
            resources: vec![],
        }
    }

    pub fn add_question(&mut self, question: Question) -> &mut Self {
        self.questions.push(question);
        self.header.n_question += 1;
        self
    }

    pub fn add_resource(&mut self, resource: ResourceRecord) -> &mut Self {
        self.resources.push(resource);
        self.header.n_answer += 1;
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = self.header.to_vec();
        for q in &self.questions {
            let mut v = q.to_vec();
            res.append(&mut v);
        }
        for r in &self.resources {
            let mut v = r.to_vec();
            res.append(&mut v);
        }
        res
    }

    pub fn parse(buff: Vec<u8>) -> Result<DNSMessage, *const str> {
        let mut message = Self::new();

        let buff = &buff[..];
        message.header = Header::parse(buff)?;

        let mut buff = &buff[Header::SIZE..];
        for _ in 0..message.header.n_question {
            let (question, next_idx) = Question::parse(buff)?;
            message.questions.push(question);
            buff = &buff[next_idx..];
        }

        // println!("Parse message: {:?}", message);
        // println!("BIN: {:0<8b}", message.header.flags);
        // println!("{:?}", message.header.get_qr());
        // println!("{:?}", message.header.get_opcode());
        // println!("{:?}", message.header.get_aa());
        // println!("{:?}", message.header.get_tc());
        // println!("{:?}", message.header.get_rd());
        // println!("{:?}", message.header.get_ra());
        // println!("{:?}", message.header.get_rcode());

        Ok(message)
    }
}
