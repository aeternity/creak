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

use serde_rlp::ser::to_bytes;

pub mod aenode;
pub mod messages;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XK_25519_ChaChaPoly_BLAKE2b".parse().unwrap();
}

fn main() {
    let data = [248, 155, 184, 153, 248, 151, 11, 1, 248, 66, 184, 64, 211, 168, 199, 217, 219, 1, 196, 183, 29, 53, 13, 172, 129, 14, 81, 59, 178, 136, 219, 35, 232, 177, 7, 138, 253, 180, 92, 161, 149, 243, 135, 250, 245, 98, 8, 52, 107, 194, 59, 47, 12, 145, 184, 95, 166, 140, 72, 36, 32, 190, 127, 185, 167, 33, 201, 143, 13, 65, 41, 1, 163, 13, 50, 15, 184, 79, 248, 77, 33, 1, 161, 1, 140, 45, 15, 171, 198, 112, 76, 122, 188, 218, 79, 128, 14, 175, 238, 64, 9, 82, 93, 44, 169, 176, 237, 27, 115, 221, 101, 211, 5, 168, 169, 235, 130, 72, 3, 161, 3, 63, 238, 194, 193, 81, 127, 40, 215, 26, 246, 178, 60, 137, 252, 206, 137, 251, 31, 160, 226, 170, 240, 1, 105, 100, 60, 3, 182, 227, 229, 110, 249, 130, 65, 40, 128];
    let r = rlp::Rlp::new(&data);
    println!("Element count: {}", r.item_count().unwrap());
    expand_rlp(&r, 0);
    let s = rlp::Rlp::new(r.at(0).unwrap().as_raw());
    expand_rlp(&s, 0);
}

fn expand_rlp(r: &rlp::Rlp, depth: u16)
{
    let mut i = 0;

    loop {
        let x = match r.at(i) {
            Ok(i) => i,
            Err(e) => {
                println!("Error at element {}: {}", i, e);
                break;
            },
        };
        for j in 0 .. depth {
            print!(" ");
        }
        if x.is_data() {
            println!("{}: data: {:?}", i, x.data().unwrap());
        } else if x.is_empty() {
            println!("{}: empty", i);
        } else if x.is_int() {
            println!("{}: int: {:?}", i, x.val_at::<u32>(i));
        } else if x.is_null() {
            println!("{}: null", i);
        } else  if x.is_list() {
            expand_rlp(&x, depth+1);
        }
        i = i + 1;
    }
}

pub fn connect() {
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
    println!("rlp1: {:?}", rlp);
    let rlp2 = to_bytes(&ping);
    println!("rlp2: {:?}", rlp2);
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
    let mut x = String::from("");
    for i in 2 .. len {
        if data[i] > 32u8 && data[i] < 127u8 {
            x.push(data[i] as char);
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
    stream.read(&mut msg_len_buf)?;
    let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
    let mut msg = vec![0u8; msg_len];
    stream.read(&mut msg[..])?;
    Ok(msg)
}
