pub struct RecordType {}

impl RecordType {
    pub const A: u16 = 1;
    pub const NS: u16 = 2;
    pub const MD: u16 = 3;
    pub const MF: u16 = 4;
    pub const CNAME: u16 = 5;
    pub const SOA: u16 = 6;
    pub const MB: u16 = 7;
    pub const MG: u16 = 8;
    pub const MR: u16 = 9;
    pub const NULL: u16 = 10;
    pub const KWS: u16 = 11;
    pub const PTR: u16 = 12;
    pub const HINFO: u16 = 13;
    pub const MINFO: u16 = 14;
    pub const MX: u16 = 15;
    pub const TXT: u16 = 16;
}

pub struct ClassCode {}

impl ClassCode {
    // the Internet
    pub const IN: u16 = 1;
    // the CSNET class (obsolete)
    pub const CS: u16 = 2;
    // the CHAOS class
    pub const CH: u16 = 3;
    // Hesiod
    pub const HS: u16 = 4;
}
