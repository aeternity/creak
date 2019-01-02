
use rlp::RlpStream;

pub struct Ping {
    port: u16,
    share: u64,
    genesis_hash: Vec<u8>,
    difficulty: u64,
    top_hash: Vec<u8>,
    sync_allowed: bool,
    peers: Vec<u8>,
}

impl Ping {
    pub fn new(
        port: u16,
        share: u64,
        genesis_hash: Vec<u8>,
        difficulty: u64,
        top_hash: Vec<u8>,
        sync_allowed: bool,
        peers: Vec<u8>) -> Ping {
        Ping{port, share, genesis_hash, difficulty, top_hash,
             sync_allowed, peers}
    }

    pub fn rlp(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        stream.append(&1u16).
            append(&self.port).
            append(&self.share).
            append(&self.genesis_hash).
            append(&self.difficulty).
            append(&self.top_hash).
            append(if self.sync_allowed { &1u16} else { &0u16}).
            append(&self.peers);
        stream.out()
    }
}
