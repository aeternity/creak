use byteorder::*;
use rlp::{Rlp, RlpStream};

use serde_rlp::ser::to_bytes;

type RlpError = Box<std::error::Error>;

use std::ops::{Index};
use std::convert::From;

const MsgFragment: u16 = 0;
const MsgP2pResponse: u16 = 100;
const MsgPing: u16 = 1;
const MsgGetHeaderByHash: u16 = 3;
const MsgGetHeaderByHeight: u16 = 15;
const MsgHeader: u16 = 4;
const MsgGetNSuccessors: u16 = 5;
const MsgHeaderHashes: u16 = 6;
const MsgGetBlockTxs: u16 = 7;
const MsgGetGeneration: u16 = 8;
const MsgTxs: u16 = 9;
const MsgBlockTxs: u16 = 13;
const MsgKeyBlock: u16 = 10;
const MsgMicroBlock: u16 = 11;
const MsgGeneration: u16 = 12;
const MsgTxPoolSyncInit: u16 = 20;
const MsgTxPoolSyncUnfold: u16 = 21;
const MsgTxPoolSyncGet: u16 = 22;
const MsgTxPoolSyncFinish: u16 = 23;
const MsgClose: u16 = 127;

fn display_message(msg_data: &Rlp) -> Result<(), RlpError> {
    println!("Starting message with {} elements:", msg_data.item_count()?);
    let mut i = msg_data.iter();
    loop {
        let ele;
        match i.next() {
            Some(x) => ele = x,
            None => break,
        };
        match ele.prototype().unwrap() {
            rlp::Prototype::Data(size) => println!("Data, size is {} content is {:?}",
                                                   size, ele.data().unwrap()),
            rlp::Prototype::List(count) => println!("List, length is {}", count),
            _ => println!("Something else"),
        };
    }
    println!("End message");
    Ok(())
}

pub fn handle_message(msg_type: u16, msg_data: &Rlp) -> Result<(), RlpError> {
    display_message(&msg_data);
    match msg_type {
        MsgP2pResponse => handle_p2p_response(&msg_data)?,
        MsgTxPoolSyncInit => handle_tx_pool_sync_init(&msg_data)?,
        MsgTxs => handle_txs(&msg_data)?,
        MsgKeyBlock => handle_key_blocks(&msg_data)?,
        MsgMicroBlock => handle_micro_block(&msg_data)?,
        _ => (),
    }
    Ok(())
}

/*
Message is RLP encoded, fields:

Result :: bool - true means ok, false means error.
Type :: int - the type of the response
Reason :: byte_array - Human readable (UTF8) reason (only set if Result is false)*
Object :: byte_array - an object of type Type if Result is true.
*/
fn handle_p2p_response(msg_data: &Rlp) -> Result<(), RlpError> {
    let version: u8 = msg_data.val_at(0)?;
    let result: u8 = msg_data.val_at(1)?;
    let _type: u8 = msg_data.val_at(2)?;
    let reason: Vec<u8> = msg_data.val_at(3)?;
    let object: Vec<u8> = msg_data.val_at(4)?;
    println!(
        "p2p_response: version: {} result: {} type: {}, reason {:?} object: {:?}",
        version, result, _type, reason, object
    );
    let r = rlp::Rlp::new(&object);
    display_message(&r);
    Ok(())
}

/*
Message has no body.
*/
fn handle_tx_pool_sync_init(msg_data: &Rlp) -> Result<(), RlpError> {
    Ok(())
}

/*
Message is RLP encoded, fields:

MicroBlock :: byte_array - Serialized micro block
Light :: bool - flag if micro block is light or normal
A normal micro block is serialized. A light micro block is serialized using aec_peer_connection:serialize_light_micro_block/1 - in effect replacing the list of serialized signed transactions with a list of transaction hashes.
*/
fn handle_micro_block(msg_data: &Rlp) -> Result<(), RlpError> {
    let _version: u8 = msg_data.val_at(0)?;
    let _data = msg_data.at(1)?.data()?;
    let payload = &rlp::Rlp::new(&_data);
    let _light: u8 = msg_data.val_at(2)?;
    let mb = MicroBlockHeader::new_from_byte_array(&payload.at(2)?.data()?)?;
    println!("{}", mb.to_string()?);
    Ok(())
}

/*
#[test]
fn test_handle_micro_block() {
    let msg_data = include!("../data/micro-block.rs");
    display_message(&msg_data).unwrap();
    handle_micro_block(&msg_data).unwrap();
    println!("Done");
}
*/

/*
 *
Message is RLP encoded, fields:

KeyBlock :: byte_array - Serialized key block
The key block is serialized.
*/
fn handle_key_blocks(msg_data: &Rlp) -> Result<(), RlpError> {
    assert!(msg_data.item_count()? % 2 == 0); // each KB is 2 nessages
    for i in 0..(msg_data.item_count()? / 2) {
        assert!(msg_data.val_at::<u16>(2 * i)? == 1);
        let data = msg_data.at(2 * i + 1)?.data()?;
        handle_key_block(&data)?
    }
    Ok(())
}

fn handle_key_block(binary: &[u8]) -> Result<(), RlpError> {
    let kb = KeyBlock::new_from_byte_array(binary)?;
    println!("height: {}", kb.height);
    println!("{}", kb.to_string()?);
    Ok(())
}

/*
#[test]
fn test_handle_keyblocks() {
    let msg_data = include!("../data/key-block.rs");
    handle_key_blocks(&msg_data).unwrap();
}
*/
/*

Message is RLP encoded, fields:

Txs:: [byte_array]
A signed transaction is serialized as a tagged and versioned signed transaction.
*/
pub fn handle_txs(msg_data: &Rlp) -> Result<(), RlpError> {
    let version: u8 = msg_data.at(0)?.data()?[0];
    assert!(version == 1);
    let tmp = msg_data.at(1)?; // temp variable so it doesn't go out of scope
    let mut iter = tmp.iter();
    let mut tx: rlp::Rlp;
    loop {
        let signed_tx = match iter.next() {
            Some(x) => x,
            None => break,
        };
        let tag = signed_tx.at(0)?.data()?[0];
        assert!(tag == 11);
        let version = signed_tx.at(1)?.data()?[0];
        assert!(version == 1);
        //signatures
        let transaction = rlp::Rlp::new(signed_tx.at(3)?.data()?);
        let tag: u8 = transaction.at(0)?.data()?[0];
        let version: u8 = transaction.at(1)?.data()?[1];
//        process_transaction(tag, &transaction);
    }
    Ok(())
}

#[derive(Debug)]
enum RlpVal {
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

trait FromRlp {
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

#[test]
fn test_handle_txs() {
    let txs = include!("../data/transactions.rs");
    display_message(&txs);
    let tmp = txs.at(1).unwrap();
    let mut iter = tmp.iter();
    let mut tx: rlp::Rlp;
    loop {
        let tx = match iter.next() {
            Some(x) => x,
            None => break,
        };
        let payload = rlp::Rlp::new(tx.data().unwrap());
        let rlp_val = RlpVal::from_rlp(&payload);
        println!("rlp_val: {:?}", rlp_val);
        display_message(&payload);
        let unknown = rlp::Rlp::new(payload.at(3).unwrap().data().unwrap());
        let tx_ = RlpVal::from_rlp(&unknown).unwrap();
        println!("tx0 {:?}", tx_[0]);
        println!("rlp_val: {:?}", tx_);
        let _u: u32 = u32::convert(&tx_[0]);
        println!("tag: {}", _u);
        display_message(&unknown).unwrap();
        let unknown2 = unknown.at(8).unwrap().data().unwrap();
        println!("Payload is {}", String::convert(&tx_[8]));
    }
}

pub fn bigend_u16(num: u16) -> Result<Vec<u8>, RlpError> {
    let mut v = vec![];
    v.write_u16::<BigEndian>(num)?;
    Ok(v)
}

/*
 * æternity expects RLP w/ some changes from the Parity
 */
pub fn mangle_rlp(data: &Vec<u8>) -> Vec<u8> {
    data.iter()
        .map(|x| if *x == 128 { 0 } else { *x })
        .collect()
}

pub struct MicroBlockHeader {
    version: u32,
    tags: [u8; 4],
    height: u64,
    prev_hash: [u8; 32],
    prev_key_hash: [u8; 32],
    state_hash: [u8; 32],
    txs_hash: [u8; 32],
    time: u64,
    fraud_hash: Option<[u8; 32]>,
    signature: [u8; 64],
}

/*
Fieldname	Size (bytes)
version	32 bits
micro_tag	1 bit
has_fraud	1 bit
unused_flags	30 bits (all set to 0)
height	8
prev_hash	32
prev_key_hash	32
state_hash	32
txs_hash	32
time	8
fraud_hash	0 or 32
signature	64
*/
impl MicroBlockHeader {
    fn new_from_byte_array(bytes: &[u8]) -> Result<MicroBlockHeader, RlpError> {
        println!("new mb from bytes: {:?}", bytes);

        let bytes = bytes.clone();
        let flags = array_ref![bytes, 4, 1][0];
        let micro = flags & 0b10000000u8;
        let has_fraud = flags & 0b01000000u8 != 0;

        Ok(MicroBlockHeader {
            version: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[0..4]).clone())?,
            tags: array_ref![bytes, 4, 4].clone(),
            height: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[8..16]).clone())?,
            prev_hash: array_ref![bytes, 16, 32].clone(),
            prev_key_hash: array_ref![bytes, 48, 32].clone(),
            state_hash: array_ref![bytes, 80, 32].clone(),
            txs_hash: array_ref![bytes, 112, 32].clone(),
            time: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[144..152]).clone())?,
            fraud_hash: if has_fraud {
                Some(array_ref![bytes, 152, 32].clone())
            } else {
                None
            },
            signature: if has_fraud {
                array_ref![bytes, 184, 64].clone()
            } else {
                array_ref![bytes, 152, 64].clone()
            },
        })
    }

    pub fn to_string(&self) -> Result<String, RlpError> {
        Ok(format!(
            "version: {} flags: {:?} height: {} prev_hash: {:?} prev_key_hash: {:?} state_hash: {:?} \
             txs_hash: {:?} time: {} fraud_hash {:?}",
            self.version,
            self.tags,
            self.height,
            self.prev_hash,
            self.prev_key_hash,
            self.state_hash,
            self.txs_hash,
            self.time,
            self.fraud_hash,
        ))
    }
}

pub struct KeyBlock {
    version: u32,
    key_unused: u32,
    height: u64,
    prev_hash: [u8; 32],
    prev_key_hash: [u8; 32],
    state_hash: [u8; 32],
    miner: [u8; 32],
    beneficiary: [u8; 32],
    target: u32,
    pow: [u8; 168],
    nonce: u64,
    time: u64,
}

/*
Fieldname	Size (bytes)
version	32 bits
key_tag	1 bit
unused_flags	31 bits (all set to 0)
height	8
prev_hash	32
prev_key_hash	32
state_hash	32
miner	32
beneficiary	32
target	4
pow	168
nonce	8
time	8
*/
impl KeyBlock {
    fn new_from_byte_array(bytes: &[u8]) -> Result<KeyBlock, RlpError> {
        println!("bytes: {:?} length {}", bytes, bytes.len());
        let bytes = bytes.clone();
        Ok(KeyBlock {
            version: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[0..4]).clone())?,
            key_unused: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[4..8]).clone())?,
            height: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[8..16]).clone())?,
            prev_hash: array_ref![bytes, 16, 32].clone(),
            prev_key_hash: array_ref![bytes, 48, 32].clone(),
            state_hash: array_ref![bytes, 80, 32].clone(),
            miner: array_ref![bytes, 112, 32].clone(),
            beneficiary: array_ref![bytes, 144, 32].clone(),
            target: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[176..180]).clone())?,
            pow: array_ref![bytes, 180, 168].clone(),
            nonce: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[348..356]).clone())?,
            time: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[356..364]).clone())?,
        })
    }

    pub fn to_string(&self) -> Result<String, RlpError> {
        Ok(format!(
            "version: {} flags: {} height: {} prev_hash: {:?} prev_key_hash: {:?} state_hash: {:?} \
             miner: {:?} beneficiary: {:?} target: {} pow: {:?} nonce: {} time: {}",
            self.version,
            self.key_unused,
            self.height,
            self.prev_hash,
            self.prev_key_hash,
            self.state_hash,
            self.miner,
            self.beneficiary,
            self.target,
            self.pow.to_vec(),
            self.nonce,
            self.time
        ))
    }
}

#[derive(Debug, Serialize)]
pub struct Ping {
    version: u16,
    port: u16,
    share: u16,
    genesis_hash: Vec<u8>,
    difficulty: u64,
    top_hash: Vec<u8>,
    sync_allowed: u16,
    peers: Vec<u8>,
}

impl Ping {
    pub fn new(
        port: u16,
        share: u16,
        genesis_hash: Vec<u8>,
        difficulty: u64,
        top_hash: Vec<u8>,
        sync_allowed: bool,
        peers: Vec<u8>,
    ) -> Ping {
        Ping {
            version: 1,
            port,
            share,
            genesis_hash,
            difficulty,
            top_hash,
            sync_allowed: if sync_allowed { 1 } else { 0 },
            peers,
        }
    }

    pub fn rlp(&self) -> Result<Vec<u8>, Box<std::error::Error>> {
        let mut stream = RlpStream::new();
        let peers: Vec<u8> = vec![];
        stream.begin_list(8).
            append(&1u16). // version
            append(&self.port).
            append(&self.share).
            append(&self.genesis_hash).
            append(&self.difficulty).
            append(&self.top_hash).
            append(&self.sync_allowed).
            begin_list(0);
        let v: Vec<u8> = stream.out();
        let mut v = mangle_rlp(&v);
        let version = bigend_u16(1)?;
        v.insert(0, version[0]); // message type
        v.insert(1, version[1]);
        println!("{:?}", v);
        Ok(v)
    }
}
