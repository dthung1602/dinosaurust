use bitflags::{bitflags, Flags};

bitflags! {
    pub struct FlagQR: u16 {
        const RESET = 0b_0_000000000000000;
        const Q = 0b_0_000000000000000;
        const R = 0b_1_000000000000000;
    }
}

bitflags! {
    pub struct FlagOpcode: u16 {
        const RESET = 0b0_0000_00000000000;
        const QUERY = 0b0_0000_00000000000;
        const IQUERY = 0b0_0001_00000000000;
        const STATUS = 0b0_0010_00000000000;
    }
}

bitflags! {
    pub struct FlagAA: u16 {
        const RESET = 0b00000_0_0000000000;
        const FALSE = 0b00000_0_0000000000;
        const TRUE = 0b00000_1_0000000000;
    }
}

bitflags! {
    pub struct FlagTC: u16 {
        const RESET = 0b000000_0_000000000;
        const FALSE = 0b000000_0_000000000;
        const TRUE = 0b000000_1_000000000;
    }
}

bitflags! {
    pub struct FlagRD: u16 {
        const RESET = 0b0000000_0_00000000;
        const FALSE = 0b0000000_0_00000000;
        const TRUE = 0b0000000_1_00000000;
    }
}

bitflags! {
    pub struct FlagRA: u16 {
        const RESET = 0b00000000_0_0000000;
        const FALSE = 0b00000000_0_0000000;
        const TRUE = 0b00000000_1_0000000;
    }
}

bitflags! {
    pub struct FlagRCode: u16 {
        const RESET = 0b000000000000_0000;
        const NOERROR = 0b000000000000_0000;
        const FORMERR = 0b000000000000_0001;
        const SERVFAIL = 0b000000000000_0010;
        const NXDOMAIN = 0b000000000000_0011;
    }
}

bitflags! {
    pub struct FlagRecordType: u16 {
        const A = 1;
        const NS = 2;
        const MD = 3;
        const MF = 4;
        const CNAME = 5;
        const SOA = 6;
        const MB = 7;
        const MG = 8;
        const MR = 9;
        const NULL = 10;
        const KWS = 11;
        const PTR = 12;
        const HINFO = 13;
        const MINFO = 14;
        const MX = 15;
        const TXT = 16;
    }
}

bitflags! {
    pub struct FlagClassCode: u16 {
        const IN = 1;
        const CS = 2;
        const CH = 3;
        const HS = 4;
    }
}
