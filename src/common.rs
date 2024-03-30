use bitflags::bitflags;

#[derive(Debug)]
pub struct FlagQR(u16);

bitflags! {
    impl  FlagQR: u16 {
        const RESET = 0b_0_111111111111111;
        const Q = 0b_0_000000000000000;
        const R = 0b_1_000000000000000;
    }
}

#[derive(Debug)]
pub struct FlagOpcode(u16);

bitflags! {
    impl FlagOpcode: u16 {
        const RESET = 0b1_0000_11111111111;
        const QUERY = 0b0_0000_00000000000;
        const IQUERY = 0b0_0001_00000000000;
        const STATUS = 0b0_0010_00000000000;
    }
}

#[derive(Debug)]
pub struct FlagAA(u16);

bitflags! {
    impl FlagAA: u16 {
        const RESET = 0b11111_0_1111111111;
        const FALSE = 0b00000_0_0000000000;
        const TRUE = 0b00000_1_0000000000;
    }
}

#[derive(Debug)]
pub struct FlagTC(u16);

bitflags! {
    impl FlagTC: u16 {
        const RESET = 0b111111_0_111111111;
        const FALSE = 0b000000_0_000000000;
        const TRUE = 0b000000_1_000000000;
    }
}

#[derive(Debug)]
pub struct FlagRD(u16);

bitflags! {
    impl FlagRD: u16 {
        const RESET = 0b1111111_0_11111111;
        const FALSE = 0b0000000_0_00000000;
        const TRUE = 0b0000000_1_00000000;
    }
}

#[derive(Debug)]
pub struct FlagRA(u16);

bitflags! {
    impl FlagRA: u16 {
        const RESET = 0b11111111_0_1111111;
        const FALSE = 0b00000000_0_0000000;
        const TRUE = 0b00000000_1_0000000;
    }
}

#[derive(Debug)]
pub struct FlagRCode(u16);

bitflags! {
    impl FlagRCode: u16 {
        const RESET = 0b111111111111_0000;
        const NOERROR = 0b000000000000_0000;
        const FORMERR = 0b000000000000_0001;
        const SERVFAIL = 0b000000000000_0010;
        const NXDOMAIN = 0b000000000000_0011;
    }
}

#[derive(Debug)]
pub struct FlagRecordType(u16);

bitflags! {
    impl FlagRecordType: u16 {
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

#[derive(Debug)]
pub struct FlagClassCode(u16);

bitflags! {
    impl FlagClassCode: u16 {
        const IN = 1;
        const CS = 2;
        const CH = 3;
        const HS = 4;
    }
}

#[derive(Debug, Clone)]
pub struct LabelSeq {
    labels: Vec<String>,
}

pub const MAX_LABEL_LEN: usize = 63;

impl LabelSeq {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = vec![];
        for label in &self.labels {
            let bytes = label.as_bytes();
            let len = bytes.len() as u8;
            res.push(len);
            res.extend_from_slice(bytes);
        }
        res
    }

    pub fn from_string(s: String) -> Result<LabelSeq, *const str> {
        let mut labels = vec![];

        for part in s.split('.') {
            if part.len() > MAX_LABEL_LEN {
                return Err("label is too long");
            }
            labels.push(part.to_string());
        }

        Ok(LabelSeq { labels })
    }

    // Return: Tuple(LabelSeq, next_index_of_buff_to_parse)
    pub fn parse(buff: &[u8]) -> Result<(LabelSeq, usize), *const str> {
        let mut labels = vec![];
        let mut i = 0;
        let last_buff_idx = buff.len() - 1;

        loop {
            // Example:
            // label_len = 4
            // | i |   |   |   |label_last_idx|

            if i > last_buff_idx {
                return Err("cannot read label length");
            }

            let label_len = buff[i] as usize;
            if label_len == 0 {
                let label_seq = LabelSeq { labels };
                return Ok((label_seq, i + 1));
            }
            if label_len > MAX_LABEL_LEN {
                return Err("label is too long");
            }

            let label_last_idx = i + label_len;
            if label_last_idx > last_buff_idx {
                return Err("cannot read label");
            }
            let slice = Vec::from(&buff[i + 1..=label_last_idx]);
            let label = String::from_utf8(slice).unwrap();
            labels.push(label);

            i = label_last_idx + 1;
        }
    }
}
