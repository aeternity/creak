use byteorder::*;
use rlp::RlpStream;

type RlpError = Box<std::error::Error>;

pub fn bigend_u16(num: u16) -> Result<Vec<u8>, RlpError>
{
    let mut v = vec![];
    v.write_u16::<BigEndian>(num)?;
    Ok(v)
}

pub struct Ping {
    port: u16,
    share: u16,
    genesis_hash: Vec<u8>,
    difficulty: u64,
    top_hash: Vec<u8>,
    sync_allowed: bool,
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
        Ping{port, share, genesis_hash, difficulty, top_hash,
             sync_allowed, peers}
    }

    pub fn rlp(&self) -> Result<Vec<u8>, Box<std::error::Error>> {
        let mut stream = RlpStream::new();
        stream.append(&1u16).
            append(&bigend_u16(self.port)?).
            append(&self.share).
            append(&self.genesis_hash).
            append(&self.difficulty).
            append(&self.top_hash).
            append(if self.sync_allowed { &1u16} else { &0u16}).
            append(&self.peers);
        Ok(stream.out())
    }
}
