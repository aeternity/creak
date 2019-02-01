use base58::ToBase58;
use byteorder::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rlp::{Rlp, RlpStream};
use serde::ser::{Serialize, Serializer};
use std::ops::{Index};
use std::convert::From;
use std::fmt;

type RlpError = Box<std::error::Error>;

#[derive(Debug)]
pub enum RlpVal {
    Val { data: Vec<u8> },
    List { data: Vec<RlpVal> },
    None,
}


impl RlpVal {

    /*
    let _test = rlp::Rlp::new(data);
    if _test.is_list() {
    let data_copy = data.clone();
    let _rlp = std::boxed::Box(rlp::Rlp::new(&data_copy));
    return &RlpVal::from_rlp(&_rlp).unwrap(); // TODO: fix
     */


    pub fn from_rlp(r: &Rlp) -> Result<RlpVal, RlpError>
    {
        if r.is_list() {
            println!("is_list");
            let mut data = Vec::<RlpVal>::new();
            let mut iter = r.iter();
            loop {
                match iter.next() {
                    Some(x) => {
                        println!("adding {:?}", x);
                        data.push(RlpVal::from_rlp(&x).unwrap());
                    },
                    None => break,
                };
            }
            Ok(RlpVal::List { data })
        } else {
            println!("not_list");
            Ok(  RlpVal::Val { data: r.data()?.to_vec() })
        }
    }
}

pub trait FromRlp {
    fn convert(val: &RlpVal) -> Self;
}

fn ensure_vec_len(v: &mut Vec<u8>, len: usize) -> &Vec<u8>{
    loop {
        if v.len() >= len { break; }
        v.insert(0,0);
    }
    v
}

impl FromRlp for u128 {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                BigEndian::read_u128(&ensure_vec_len(&mut data.clone(), 16))
            },
            _ => 0
        }
    }
 }

impl FromRlp for u64 {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                BigEndian::read_u64(&ensure_vec_len(&mut data.clone(), 8))
            },
            _ => 0
        }
    }
 }

impl FromRlp for u32 {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                BigEndian::read_u32(&ensure_vec_len(&mut data.clone(), 4))
            },
            _ => 0
        }
    }
 }

impl FromRlp for u16 {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                BigEndian::read_u16(&ensure_vec_len(&mut data.clone(), 2))
            },
            _ => 0
        }
    }
}

impl FromRlp for String {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                match String::from_utf8(data.to_vec()) {
                    Ok(x) => x,
                    Err(e) => String::from(e.to_string()),
                }
            },
            _ => String::from(""),
        }
    }
}

impl FromRlp for AeIdentifier {
    fn convert(item: &RlpVal) -> Self {
        match item {
            RlpVal::Val { data } => {
                return match AeIdentifier::from_bytes(&data.to_vec()) {
                    Some(x) => x,
                    None => AeIdentifier { id: String::from("")},
                };
            },
            _ => AeIdentifier { id: String::from("") },
        }
    }
}


pub struct AeIdentifier {
    id: String,
}

impl AeIdentifier {
    pub fn from_bytes(bytes: &Vec<u8>) -> Option<AeIdentifier>
    {
        let prefix = match bytes[0] {
            1 => "ak_",
            2 => "nm_",
            3 => "cm_",
            4 => "ok",
            5 => "ct_",
            6 => "ch_",
            _ => return None,
        };
        Some(AeIdentifier{ id: format!("{}{}", prefix, to_base58check(&bytes[1..])) })
    }
}

impl std::fmt::Display for AeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Serialize for AeIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.id)
    }
}


impl Index<usize> for RlpVal {
    type Output = RlpVal;

    fn index(&self, index: usize) -> &RlpVal {
        match self {
            RlpVal::List { data } => &data[index],
            RlpVal::Val { data }  => {
                &RlpVal::None
            },
            _ => &RlpVal::None,
        }
    }
}


/* taken from https://github.com/dotcypress/base58check
 * reproduced with kind permission of the author
 */
fn to_base58check(data: &[u8]) -> String {
    let mut payload = data.to_vec();
    let mut checksum = double_sha256(&payload);
    payload.append(&mut checksum[..4].to_vec());
    payload.to_base58()
}

/*
 * taken from https://github.com/dotcypress/base58check
 * reproduced with kind permission of the author
 */
fn double_sha256(payload: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut hash = vec![0; hasher.output_bytes()];
    hasher.input(&payload);
    hasher.result(&mut hash);
    hasher.reset();
    hasher.input(&hash);
    hasher.result(&mut hash);
    hash.to_vec()
}
