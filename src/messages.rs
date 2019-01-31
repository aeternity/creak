use byteorder::*;
use rlp::{Rlp, RlpStream, };

use serde_rlp::ser::to_bytes;

type RlpError = Box<std::error::Error>;

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

fn display_message(msg_data: &Rlp) -> Result<(), RlpError>
{
    println!("Starting message with {} elements:", msg_data.item_count()?);
    let mut i = msg_data.iter();
    loop {
        let ele;
        match i.next() {
            Some(x) => ele = x,
            None => break,
        };
        match ele.prototype().unwrap(){
            rlp::Prototype::Data(size) => println!("Data, size is {}", size),
            rlp::Prototype::List(count) => println!("List, length is {}", count),
            _ => println!("Something else"),
        };

    }
    println!("End message");
    Ok(())
}


pub fn handle_message(msg_type: u16, msg_data: &Rlp) -> Result<(), RlpError>
{
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
fn handle_p2p_response(msg_data: &Rlp) -> Result<(), RlpError>
{
    println!("p2p_response: {:?}", msg_data.as_raw());
    Ok(())
}

/*
Message has no body.
*/
fn handle_tx_pool_sync_init(msg_data: &Rlp) -> Result<(), RlpError>
{
    Ok(())
}

/*
Message is RLP encoded, fields:

MicroBlock :: byte_array - Serialized micro block
Light :: bool - flag if micro block is light or normal
A normal micro block is serialized. A light micro block is serialized using aec_peer_connection:serialize_light_micro_block/1 - in effect replacing the list of serialized signed transactions with a list of transaction hashes.
*/
fn handle_micro_block(msg_data: &Rlp) -> Result<(), RlpError>
{

    // let version: &[u8] = msg_data.at(0)?.data()?;
    // println!("Version: {:?}",version);
    // let payload: Rlp = msg_data.at(1)?;
    // let light: u16 = msg_data.at(2)?.as_val().unwrap();
    // println!("TXs: {:?}", payload.item_count()?);
    // let tx = Rlp::new(payload.at(0)?.as_raw());
    // println!("TX tag is {:?}", tx.at(0)?);
    Ok(())
}

/*
 *
Message is RLP encoded, fields:

KeyBlock :: byte_array - Serialized key block
The key block is serialized.
*/
fn handle_key_blocks(msg_data: &Rlp) -> Result<(), RlpError>
{
    assert!(msg_data.item_count()? % 2 == 0); // each KB is 2 nessages
    for i in 0 .. (msg_data.item_count()? / 2) {
        assert!(msg_data.val_at::<u16>(2*i)? == 1);
        let data = msg_data.at(2*i + 1)?.data()?;
        handle_key_block(&data)?
    }
    Ok(())
}

fn handle_key_block(binary: &[u8]) -> Result<(), RlpError>
{
    let kb = KeyBlock::new_from_byte_array(binary)?;
    println!("height: {}", kb.height);
    println!("{}", kb.to_string()?);
    Ok(())
}

#[test]
fn test_handle_keyblocks() {
    let msg_data = rlp::Rlp::new(&[249, 1, 112, 1, 185, 1, 108, 0, 0, 0, 1, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 213, 92, 221, 122, 51, 146, 97, 216, 168, 22, 168, 250, 143, 2, 231, 167, 245, 229, 210, 246, 69, 182, 88, 13, 250, 162, 188, 15, 14, 101, 248, 148, 29, 92, 221, 122, 51, 146, 97, 216, 168, 22, 168, 250, 143, 2, 231, 167, 245, 229, 210, 246, 69, 182, 88, 13, 250, 162, 188, 15, 14, 101, 248, 148, 29, 176, 41, 10, 83, 206, 164, 90, 232, 160, 91, 251, 73, 66, 64, 89, 6, 188, 12, 187, 151, 130, 18, 175, 172, 230, 244, 62, 185, 65, 99, 228, 87, 89, 208, 74, 214, 149, 81, 254, 173, 191, 153, 147, 65, 203, 7, 189, 33, 181, 98, 226, 184, 225, 46, 215, 193, 37, 70, 29, 232, 38, 124, 159, 81, 104, 65, 117, 174, 49, 251, 29, 202, 69, 174, 147, 56, 60, 150, 188, 247, 149, 85, 150, 148, 88, 102, 186, 208, 87, 101, 78, 111, 189, 5, 144, 101, 30, 8, 218, 121, 0, 108, 68, 250, 2, 123, 179, 154, 2, 160, 22, 119, 2, 217, 91, 235, 3, 24, 170, 15, 5, 53, 86, 204, 6, 56, 4, 28, 7, 54, 165, 88, 8, 37, 67, 86, 8, 104, 108, 188, 8, 204, 203, 135, 12, 8, 176, 186, 12, 49, 40, 239, 12, 189, 43, 116, 13, 16, 49, 158, 13, 49, 6, 125, 14, 42, 177, 119, 14, 197, 110, 178, 14, 208, 156, 200, 15, 103, 19, 163, 16, 193, 222, 245, 17, 79, 131, 88, 17, 204, 2, 106, 18, 186, 238, 190, 19, 129, 5, 18, 20, 11, 103, 137, 20, 121, 122, 61, 20, 223, 182, 151, 22, 85, 181, 228, 23, 221, 51, 38, 24, 39, 167, 255, 24, 79, 76, 252, 24, 230, 191, 206, 27, 124, 91, 228, 28, 0, 173, 69, 29, 79, 226, 177, 29, 254, 48, 243, 30, 44, 230, 128, 30, 147, 120, 54, 31, 55, 227, 55, 31, 72, 94, 156, 31, 109, 74, 209, 184, 248, 70, 81, 19, 166, 147, 231, 0, 0, 1, 104, 162, 223, 140, 59]);
    handle_key_blocks(&msg_data).unwrap();
}

/*

Message is RLP encoded, fields:

Txs:: [byte_array]
A signed transaction is serialized as a tagged and versioned signed transaction.
*/
pub fn handle_txs(msg_data: &Rlp) -> Result<(), RlpError>
{

    let version: &[u8] = msg_data.at(0)?.data()?;
    println!("Version: {:?}",version);
    let txs: Rlp = msg_data.at(1)?;
    println!("Txs are {:?}", txs);
    for i in 0 .. txs.item_count().unwrap() {
        let stx_raw = txs.at(i).unwrap();
        let stx = Rlp::new(stx_raw.as_raw());
        let tx_raw = stx.at(3).unwrap();
        let tx = Rlp::new(tx_raw.as_raw());
        println!("Payload is {:?}", tx.at(8).unwrap());
    }

    Ok(())
}

pub fn bigend_u16(num: u16) -> Result<Vec<u8>, RlpError>
{
    let mut v = vec![];
    v.write_u16::<BigEndian>(num)?;
    Ok(v)
}

/*
 * æternity expects RLP w/ some changes from the Parity
 */
pub fn mangle_rlp(data: &Vec<u8>) -> Vec<u8>{
    data.iter().map(|x| {
        if *x == 128 {
            0
        } else {
            *x
        }
    }).collect()
}

pub struct KeyBlock {
    version: u32,
    key_unused: u32,
    height: u64,
    prev_hash: [u8;32],
    prev_key_hash: [u8;32],
    state_hash: [u8;32],
    miner: [u8;32],
    beneficiary: [u8;32],
    target: u32,
    pow: [u8;168],
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
    fn new_from_byte_array(bytes: &[u8]) -> Result<KeyBlock, RlpError>
    {
        println!("bytes: {:?} length {}", bytes, bytes.len());
        let bytes = bytes.clone();
        Ok(KeyBlock {
            version: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[0..4]).clone())?,
            key_unused: <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[4..8]).clone())?,
            height: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[8..16]).clone())?,
            prev_hash: array_ref![bytes,16,32].clone(),
            prev_key_hash: array_ref![bytes,48,32].clone(),
            state_hash: array_ref![bytes, 80, 32].clone(),
            miner: array_ref![bytes, 112, 32].clone(),
            beneficiary: array_ref![bytes, 144, 32].clone(),
            target:  <&[u8]>::read_u32::<BigEndian>(&mut (&bytes[176..180]).clone())?,
            pow: array_ref![bytes, 180, 168].clone(),
            nonce: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[348..356]).clone())?,
            time: <&[u8]>::read_u64::<BigEndian>(&mut (&bytes[356..364]).clone())?,
        })
    }

    pub fn to_string(&self) -> Result<String, RlpError>
    {
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

#[derive(Debug,Serialize)]
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
        peers: Vec<u8>) -> Ping {
        Ping{version: 1, port, share, genesis_hash, difficulty, top_hash,
             sync_allowed: if sync_allowed { 1 } else { 0 }, peers}
    }

    pub fn rlp(&self) -> Result<Vec<u8>, Box<std::error::Error>> {
        let mut stream = RlpStream::new();
        let peers: Vec<u8> = vec!();
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
