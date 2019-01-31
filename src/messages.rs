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
    Ok(())
}


pub fn handle_message(msg_type: u16, msg_data: &Rlp) -> Result<(), RlpError>
{
    display_message(&msg_data);
    match msg_type {
        MsgP2pResponse => handle_p2p_response(&msg_data)?,
        MsgTxPoolSyncInit => handle_tx_pool_sync_init(&msg_data)?,
        MsgTxs => handle_txs(&msg_data)?,
        MsgKeyBlock => handle_keyblock(&msg_data)?,
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
fn handle_keyblock(msg_data: &Rlp) -> Result<(), RlpError>
{
    let version: u32 = msg_data.val_at(0)?;
    //    let flags: u32 = msg_data.val_at(1).unwrap();
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
    Ok(())
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
