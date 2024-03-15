use crate::common::FlagQR;
use crate::header::Header;
use crate::question::Question;
use crate::resourserecord::ResourceRecord;

pub struct DNSMessage {
    header: Header,
    questions: Vec<Question>,
    resources: Vec<ResourceRecord>,
}

impl DNSMessage {
    // TODO rename reply_to(request: DNSMessage)
    pub fn new_reply() -> DNSMessage {
        let mut header = Header::new();
        header.set_qr(FlagQR::R);

        DNSMessage {
            header,
            questions: vec![],
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
        self.header.n_auth_res += 1;
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
}
