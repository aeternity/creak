extern crate byteorder;
#[macro_use]extern crate lazy_static;
extern crate regex;
extern crate rlp;
extern crate base58check;
extern crate snow;

use base58check::{FromBase58Check};
use byteorder::{BigEndian, ReadBytesExt};
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
    let node = aenode::Aenode::new(
        &String::from("aenode://pp_2YpEsVV2ZFzQQxXGChVBH8Da2xGnx5BQFw3d6KmgwgQFBcEch@127.0.0.1:3015"))
        .unwrap();
    let gen_hash = "pbtwgLrNu23k9PA6XCZnUbtsvEFeQGgavY4FS2do3QP8kcp2z".from_base58check().unwrap().1;
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
    /* handshake...
     */
    // <- s
    println!("s");
    let len = noise.write_message(&[], &mut buf).unwrap();
    send(&mut stream, &buf[..len]);

    // -> e, es
    println!("e,es");
    noise.read_message(&recv(&mut stream).unwrap(), &mut buf).unwrap();

    // <- e, ee
    println!("e,ee");
    let len = noise.write_message(&[], &mut buf).unwrap();
    send(&mut stream, &buf[..len]);

       // -> s, se
//    println!("s,se");
//    noise.read_message(&recv(&mut stream).unwrap(), &mut buf).unwrap();

    // -> e, ee
//    let len = noise.write_message(&[], &mut buf).unwrap();
//    send(&mut stream, &buf[..len]);

       // <- s, se
//    noise.read_message(&recv(&mut stream).unwrap(), &mut buf).unwrap();

    println!("Entering transport mode");
    let mut noise = noise.into_transport_mode().unwrap();
    println!("session established...");
    let mut buf = vec![0u8; 65535];
    let ping = messages::Ping::new(3015, 0, gen_hash.clone(), 0, gen_hash, true, Vec::new());
    let mut rlp = ping.rlp();
    println!("{:#?}", rlp);
    stream.write_all(&rlp).unwrap();
    loop {
        let msg = recv(&mut stream);
        match msg {
            Ok(x) => {
                match noise.read_message(&x, &mut buf) {
                    Ok(x) => println!("{:?}", buf),
                    Err(x) => println!("Error {}", x),
                };
            },
            Err(x) => println!("Error {}", x),
        };
    }

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
