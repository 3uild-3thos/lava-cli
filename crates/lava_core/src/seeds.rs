use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LavaSeed {
    String(String),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    USize(usize),
    ISize(isize),
    PublicKey(String),
}

impl LavaSeed {
    pub fn to_mocha_account(&self, pda_owner: bool) -> String {
        match self {
            LavaSeed::String(s) => format!("Buffer.from(\"{}\", \"utf-8\")", s),
            LavaSeed::U8(n) => format!("new BN({}).toBuffer(\"le\", 1)", n),
            LavaSeed::I8(n) => format!("new BN({}).toBuffer(\"le\", 1)", n),
            LavaSeed::U16(n) => format!("new BN({}).toBuffer(\"le\", 2)", n),
            LavaSeed::I16(n) => format!("new BN({}).toBuffer(\"le\", 2)", n),
            LavaSeed::U32(n) => format!("new BN({}).toBuffer(\"le\", 4)", n),
            LavaSeed::I32(n) => format!("new BN({}).toBuffer(\"le\", 4)", n),
            LavaSeed::U64(n) => format!("new BN({}).toBuffer(\"le\", 8)", n),
            LavaSeed::I64(n) => format!("new BN({}).toBuffer(\"le\", 8)", n),
            // TODO: Handle encoding properly based upon size of the usize
            LavaSeed::USize(n) => format!("new BN({}).toBuffer(\"le\", 8)", n),
            LavaSeed::ISize(n) => format!("new BN({}).toBuffer(\"le\", 8)", n),
            LavaSeed::PublicKey(n) => {
                if pda_owner {
                    format!("{}.publicKey.toBuffer()", n.to_case(Case::Camel))
                } else {
                    format!("{}.toBuffer()", n.to_case(Case::Camel))
                }
            }
        }
    }
}
