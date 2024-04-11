use crate::common::*;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub n_question: u16,
    pub n_answer: u16,
    pub n_auth_res: u16,
    pub n_addi_rrs: u16,
}

impl Header {
    pub const SIZE: usize = 12;

    pub fn new() -> Header {
        Header {
            id: rand::random(),
            flags: FlagQR::Q.bits(),
            n_question: 0,
            n_answer: 0,
            n_auth_res: 0,
            n_addi_rrs: 0,
        }
    }

    pub fn reply_to(header: &Header) -> Header {
        let mut header = Header {
            id: header.id,
            flags: header.flags,
            n_question: 0,
            n_answer: 0,
            n_auth_res: 0,
            n_addi_rrs: 0,
        };
        header.set_qr(FlagQR::R);
        header.set_aa(FlagAA::FALSE);
        header.set_tc(FlagTC::FALSE);
        header.set_ra(FlagRA::TRUE);
        header.set_rd(FlagRD::TRUE);
        header.set_ra(FlagRA::TRUE);

        header
    }

    pub fn set_qr(&mut self, qr: FlagQR) -> &mut Self {
        self.flags = self.flags & FlagQR::RESET.bits() | qr.bits();
        self
    }

    pub fn get_qr(&self) -> FlagQR {
        if self.flags >> 15 == 1 {
            FlagQR::R
        } else {
            FlagQR::Q
        }
    }

    pub fn set_opcode(&mut self, opcode: FlagOpcode) -> &mut Self {
        self.flags = self.flags & FlagOpcode::RESET.bits() | opcode.bits();
        self
    }

    pub fn get_opcode(&self) -> Result<FlagOpcode, *const str> {
        match self.flags << 1 >> 12 {
            0 => Ok(FlagOpcode::QUERY),
            1 => Ok(FlagOpcode::IQUERY),
            2 => Ok(FlagOpcode::STATUS),
            _ => Err("Invalid opcode flag"),
        }
    }

    pub fn set_aa(&mut self, aa: FlagAA) -> &mut Self {
        self.flags = self.flags & FlagAA::RESET.bits() | aa.bits();
        self
    }

    pub fn get_aa(&self) -> FlagAA {
        if self.flags << 5 >> 15 == 1 {
            FlagAA::TRUE
        } else {
            FlagAA::FALSE
        }
    }

    pub fn set_tc(&mut self, tc: FlagTC) -> &mut Self {
        self.flags = self.flags & FlagTC::RESET.bits() | tc.bits();
        self
    }

    pub fn get_tc(&self) -> FlagTC {
        if self.flags << 6 >> 15 == 1 {
            FlagTC::TRUE
        } else {
            FlagTC::FALSE
        }
    }

    pub fn set_rd(&mut self, rd: FlagRD) -> &mut Self {
        self.flags = self.flags & FlagRD::RESET.bits() | rd.bits();
        self
    }

    pub fn get_rd(&self) -> FlagRD {
        if self.flags << 7 >> 15 == 1 {
            FlagRD::TRUE
        } else {
            FlagRD::FALSE
        }
    }

    pub fn set_ra(&mut self, ra: FlagRA) -> &mut Self {
        self.flags = self.flags & FlagRA::RESET.bits() | ra.bits();
        self
    }

    pub fn get_ra(&self) -> FlagRA {
        if self.flags << 8 >> 15 == 1 {
            FlagRA::TRUE
        } else {
            FlagRA::FALSE
        }
    }

    pub fn set_rcode(&mut self, rcode: FlagRCode) -> &mut Self {
        self.flags = self.flags & FlagRA::RESET.bits() | rcode.bits();
        self
    }

    pub fn get_rcode(&self) -> Result<FlagRCode, *const str> {
        match self.flags << 12 >> 12 {
            0 => Ok(FlagRCode::NOERROR),
            1 => Ok(FlagRCode::FORMERR),
            2 => Ok(FlagRCode::SERVFAIL),
            3 => Ok(FlagRCode::NXDOMAIN),
            _ => Err("unsupported rcode flag"),
        }
    }

    pub fn serialize(&self, context: &mut SerializeContext) {
        let mut val = vec![
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
        ];
        context.append(&mut val)
    }

    pub fn parse(context: &mut ParseContext) -> Result<Header, *const str> {
        let mut header = Header {
            id: 0,
            flags: 0,
            n_question: 0,
            n_answer: 0,
            n_auth_res: 0,
            n_addi_rrs: 0,
        };

        let buff = context.current_slice();

        if buff.len() < Self::SIZE {
            return Err("header too short");
        }

        header.flags = u16::from_be_bytes([buff[2], buff[3]]);
        header.validate_flags()?;

        header.id = u16::from_be_bytes([buff[0], buff[1]]);
        header.n_question = u16::from_be_bytes([buff[4], buff[5]]);
        header.n_answer = u16::from_be_bytes([buff[6], buff[7]]);
        header.n_auth_res = u16::from_be_bytes([buff[8], buff[9]]);
        header.n_addi_rrs = u16::from_be_bytes([buff[10], buff[11]]);

        context.advance(Self::SIZE);
        Ok(header)
    }

    fn validate_flags(&self) -> Result<(), *const str> {
        self.get_opcode()?;
        self.get_rcode()?;
        Ok(())
    }
}
