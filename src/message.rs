use crate::common::{FlagRA, FlagRD, ParseContext, SerializeContext};
use crate::header::Header;
use crate::question::Question;
use crate::resourserecord::ResourceRecord;

#[derive(Debug)]
pub struct DNSMessage {
    pub header: Header,
    pub questions: Vec<Question>,
    pub resources: Vec<ResourceRecord>,
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
        header.set_rd(FlagRD::TRUE);
        header.set_ra(FlagRA::TRUE);
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

    pub fn serialize(&self) -> Vec<u8> {
        let mut context = SerializeContext::new();
        self.header.serialize(&mut context);
        for q in &self.questions {
            q.serialize(&mut context);
        }
        for r in &self.resources {
            r.serialize(&mut context);
        }

        context.to_vec()
    }

    pub fn parse(buff: Vec<u8>) -> Result<DNSMessage, *const str> {
        let mut context = ParseContext::new(buff);
        let mut message = Self::new();

        message.header = Header::parse(&mut context)?;

        for _ in 0..message.header.n_question {
            let question = Question::parse(&mut context)?;
            message.questions.push(question);
        }

        for _ in 0..message.header.n_answer {
            let resource = ResourceRecord::parse(&mut context)?;
            message.resources.push(resource);
        }

        Ok(message)
    }
}
