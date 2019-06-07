#[macro_use]
extern crate arrayref;
extern crate base58check;
extern crate byteorder;
extern crate curl;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rlp;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_rlp;
#[macro_use]
extern crate simple_error;
extern crate hex;
extern crate snow;

use base58check::FromBase58Check;
use byteorder::{BigEndian, ByteOrder};
use snow::params::NoiseParams;
use snow::Builder;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use serde_rlp::ser::to_bytes;

mod aenode;
pub mod jsonifier;
pub mod messages;
pub mod rlp_val;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XK_25519_ChaChaPoly_BLAKE2b".parse().unwrap();
}

pub fn handle_message(data: &[u8], len: usize) {
    let msg_type = BigEndian::read_u16(&data[0..2]);
    let msg = rlp::Rlp::new(&data[2..len]);
    println!("Msg type: {}", msg_type);
    println!("Msg: {:?}", msg.as_raw());
    let mut x = String::from("");
    for item in data.iter().take(len).skip(2) {
        if *item > 32u8 && *item < 127u8 {
            x.push(*item as char);
        } else {
            x.push('-');
        }
    }
    println!("Received: {}", x);
    messages::handle_message(msg_type, &msg).unwrap();
    println!("\n\n\n");
}

/// Hyper-basic stream transport sender. 16-bit BE size followed by payload.
fn send(stream: &mut TcpStream, buf: &[u8]) {
    let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
    stream.write_all(&msg_len_buf).unwrap();
    stream.write_all(buf).unwrap();
}

/// Hyper-basic stream transport receiver. 16-bit BE size followed by payload.
fn recv(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut msg_len_buf = [0u8; 2];
    let mut _read = stream.read(&mut msg_len_buf)?;
    let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
    let mut msg = vec![0u8; msg_len];
    _read = stream.read(&mut msg[..])?;
    Ok(msg)
}
