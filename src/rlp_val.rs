use byteorder::*;
use rlp::{Rlp, RlpStream};

use serde_rlp::ser::to_bytes;

type RlpError = Box<std::error::Error>;

use std::ops::{Index};
use std::convert::From;

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
        if v.len() >= 4 { break; }
        v.insert(0,0);
    }
    v
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

pub struct AeIdentifier {
    id: String,
}

impl AeIdentifier {

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
