extern crate byteorder;
#[macro_use]extern crate lazy_static;
extern crate regex;
extern crate rlp;
extern crate base58check;
extern crate snow;
extern crate hex;
#[macro_use]extern crate serde_rlp;
#[macro_use]extern crate serde_derive;

use base58check::{FromBase58Check};
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use noise_protocol::*;
use noise_protocol::patterns::*;
use rlp::encode;
use snow::Builder;
use snow::params::NoiseParams;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

pub mod aenode;
pub mod messages;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XK_25519_ChaChaPoly_BLAKE2b".parse().unwrap();
}

fn main() {
    let mut buf = vec![0u8; 65535];
    let prologue: [u8;50] = [0,0,0,0,0,0,0,1,108,21,218,110,191,175,2,120,254,175,77,241,176,241,169,130,85,7,174,123,154,73,75,195,76,145,113,63,56,221,87,131,97,101,95,109,97,105,110,110,101,116];
    let args: Vec<String> = std::env::args().collect();
    let node = aenode::Aenode::new(&args[1])
        //&String::from("aenode://pp_2kzKvxEg9NbBXn6krSeNec8kSeiJy8GXxnoTanX2zr1ffABvqd@192.168.111.81:3015"))
        .unwrap();
    let mut gen_hash = "pbtwgLrNu23k9PA6XCZnUbtsvEFeQGgavY4FS2do3QP8kcp2z".from_base58check().unwrap().1;
    gen_hash.insert(0, "pbtwgLrNu23k9PA6XCZnUbtsvEFeQGgavY4FS2do3QP8kcp2z".from_base58check().unwrap().0);

    let builder: Builder = Builder::new(PARAMS.clone());
    let keypair = builder.generate_keypair().unwrap();

    let mut pk = base58check::FromBase58Check::from_base58check(node.pub_key.as_str()).unwrap().1;
    pk.insert(0, base58check::FromBase58Check::from_base58check(node.pub_key.as_str()).unwrap().0);
    println!("{:?}", pk);

    println!("{}", node.pub_key);
    let mut noise = builder
       .prologue(&prologue)
        .local_private_key(&keypair.private)
        .remote_public_key(&pk)
        .build_initiator().unwrap();
    let mut stream = TcpStream::connect((node.address, node.port)).unwrap();
    println!("connected...");

    loop {
        if noise.is_handshake_finished() { break; }
        let len = noise.write_message(&[], &mut buf).unwrap();
        send(&mut stream, &buf[..len]);
        if noise.is_handshake_finished() { break; }
        noise.read_message(&recv(&mut stream).unwrap(), &mut buf).unwrap();
    }
    println!("Entering transport mode");
    let mut noise = noise.into_transport_mode().unwrap();
    println!("session established...");
    let mut buf = vec![0u8; 65535];
    let ping = messages::Ping::new(3015, 0, gen_hash.clone(), 0, gen_hash, true, Vec::new());
    let mut rlp = ping.rlp().unwrap();
    println!("{:?}", rlp);
    println!("{}", hex::encode(rlp.clone()));
    let len = noise.write_message(&rlp, &mut buf).unwrap();
    send(&mut stream, &mut buf[..len]);
    loop {
        let msg = recv(&mut stream);
        match msg {
            Ok(x) => {
                match noise.read_message(&x, &mut buf) {
                    Ok(x) => handle_message(&buf, x),
                    Err(x) => {
                        println!("Noise error {}", x);
                        break;
                    },
                };
            },
            Err(x) => println!("TCP error {}", x),
        };
    }

}

pub fn handle_message(data: &[u8], len: usize) {
    let msg_type = BigEndian::read_u16(&data[0..2]);
    let msg = rlp::Rlp::new(&data[2..len]);
    println!("Msg type: {}", msg_type);
    messages::handle_message(msg_type, &msg).unwrap();
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
    stream.read(&mut msg_len_buf)?;
    let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
    let mut msg = vec![0u8; msg_len];
    stream.read(&mut msg[..])?;
    Ok(msg)
}
