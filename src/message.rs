use crate::common::{ParseContext, SerializeContext};
use crate::header::Header;
use crate::question::Question;
use crate::resourserecord::ResourceRecord;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub resources: Vec<ResourceRecord>,
    pub auth_resources: Vec<ResourceRecord>, // all must be of type SOA
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: Header::new(),
            questions: vec![],
            resources: vec![],
            auth_resources: vec![],
        }
    }

    pub fn reply_to(request: &Message) -> Message {
        let mut header = Header::reply_to(&request.header);

        let questions = request.questions.clone();
        header.n_question = questions.len() as u16;

        Message {
            header,
            questions,
            resources: vec![],
            auth_resources: vec![],
        }
    }

    pub fn copy_resources(&mut self, other: &Self) {
        self.header.n_answer = other.header.n_answer;
        self.header.n_auth_res = other.header.n_auth_res;
        self.resources = other.resources.clone();
        self.auth_resources = other.auth_resources.clone();
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
        for r in &self.auth_resources {
            r.serialize(&mut context);
        }

        context.to_vec()
    }

    pub fn parse(buff: Vec<u8>) -> Result<Message, *const str> {
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

        for _ in 0..message.header.n_auth_res {
            let resource = ResourceRecord::parse(&mut context)?;
            message.auth_resources.push(resource);
        }

        Ok(message)
    }
}
