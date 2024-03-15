use bitflags::Flags;

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
    pub fn new() -> Header {
        Header {
            id: rand::random(),
            flags: 0,
            n_question: 0,
            n_answer: 0,
            n_auth_res: 0,
            n_addi_rrs: 0,
        }
    }

    pub fn set_qr(&mut self, qr: FlagQR) -> &mut Self {
        self.flags = self.flags & FlagQR::RESET.bits() | qr.bits();
        self
    }

    pub fn set_opcode(&mut self, opcode: FlagOpcode) -> &mut Self {
        self.flags = self.flags & FlagOpcode::RESET.bits() | opcode.bits();
        self
    }

    pub fn set_aa(&mut self, aa: FlagAA) -> &mut Self {
        self.flags = self.flags & FlagAA::RESET.bits() | aa.bits();
        self
    }

    pub fn set_tc(&mut self, tc: FlagTC) -> &mut Self {
        self.flags = self.flags & FlagTC::RESET.bits() | tc.bits();
        self
    }

    pub fn set_rd(&mut self, rd: FlagRD) -> &mut Self {
        self.flags = self.flags & FlagRD::RESET.bits() | rd.bits();
        self
    }

    pub fn set_ra(&mut self, ra: FlagRA) -> &mut Self {
        self.flags = self.flags & FlagRA::RESET.bits() | ra.bits();
        self
    }

    pub fn set_rcode(&mut self, rcode: FlagRCode) -> &mut Self {
        self.flags = self.flags & FlagRA::RESET.bits() | rcode.bits();
        self
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
