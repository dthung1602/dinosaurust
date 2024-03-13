#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub n_question: u16,
    pub n_answer: u16,
    pub n_auth_res: u16,
    pub n_addi_rrs: u16,
}

pub struct Flags {}

impl Flags {
    const QR_Q: u16 = 0b_0_000000000000000;
    const QR_R: u16 = 0b_1_000000000000000;

    const OPCODE_QUERY : u16 = 0b0_0000_00000000000;
    const OPCODE_IQUERY: u16 = 0b0_0001_00000000000;
    const OPCODE_STATUS: u16 = 0b0_0010_00000000000;

    const AA_FALSE: u16 = 0b00000_0_0000000000;
    const AA_TRUE : u16 = 0b00000_1_0000000000;

    const TC_FALSE: u16 = 0b000000_0_000000000;
    const TC_TRUE : u16 = 0b000000_1_000000000;

    const RD_FALSE: u16 = 0b0000000_0_00000000;
    const RD_TRUE : u16 = 0b0000000_1_00000000;

    const RA_FALSE: u16 = 0b00000000_0_0000000;
    const RA_TRUE : u16 = 0b00000000_1_0000000;

    const RCODE_NOERROR : u16 = 0b000000000000_0000;
    const RCODE_FORMERR : u16 = 0b000000000000_0001;
    const RCODE_SERVFAIL: u16 = 0b000000000000_0010;
    const RCODE_NXDOMAIN: u16 = 0b000000000000_0011;
}

impl Header {
    pub fn new_reply() -> Header {
        let flags = Flags::QR_R | Flags::OPCODE_STATUS | Flags::RCODE_NOERROR;
        Header {
            id: 1212,
            flags,
            n_question: 0,
            n_answer: 0,
            n_auth_res: 0,
            n_addi_rrs: 0,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            (self.id >> 8) as u8,
            self.id as u8,
            (self.flags >> 8) as u8,
            self.flags as u8,
            (self.n_question >> 8) as u8,
            self.n_question as u8,
            (self.n_answer >> 8) as u8,
            self.n_answer as u8,
            (self.n_auth_res >> 8) as u8,
            self.n_auth_res as u8,
            (self.n_addi_rrs >> 8) as u8,
            self.n_addi_rrs as u8,
        ]
    }
}
