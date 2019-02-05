#[macro_use] extern crate arrayref;
extern crate base58check;
extern crate byteorder;
extern crate curl;
#[macro_use]extern crate lazy_static;
extern crate regex;
extern crate rlp;
#[macro_use]extern crate serde_derive;
#[macro_use]extern crate serde_json;
#[macro_use]extern crate serde_rlp;
#[macro_use]extern crate simple_error;
extern crate snow;
extern crate hex;

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
pub mod rlp_val;
pub mod jsonifier;

lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XK_25519_ChaChaPoly_BLAKE2b".parse().unwrap();
}

fn foo() {
    let data = [249, 1, 112, 1, 185, 1, 108, 0, 0, 0, 1, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 146, 183, 94, 163, 90, 134, 253, 199, 123, 150, 6, 25, 222, 99, 125, 218, 120, 216, 35, 153, 43, 122, 51, 210, 120, 102, 98, 131, 113, 226, 235, 24, 253, 183, 94, 163, 90, 134, 253, 199, 123, 150, 6, 25, 222, 99, 125, 218, 120, 216, 35, 153, 43, 122, 51, 210, 120, 102, 98, 131, 113, 226, 235, 24, 253, 144, 208, 65, 168, 110, 128, 237, 55, 171, 155, 73, 55, 145, 125, 246, 27, 83, 245, 29, 32, 19, 21, 227, 189, 0, 70, 88, 242, 19, 140, 47, 147, 76, 252, 5, 187, 136, 10, 175, 121, 154, 43, 41, 162, 170, 65, 248, 103, 22, 38, 221, 31, 216, 55, 253, 47, 205, 197, 197, 27, 150, 214, 23, 216, 104, 65, 117, 174, 49, 251, 29, 202, 69, 174, 147, 56, 60, 150, 188, 247, 149, 85, 150, 148, 88, 102, 186, 208, 87, 101, 78, 111, 189, 5, 144, 101, 30, 10, 158, 223, 0, 196, 247, 121, 1, 115, 26, 93, 2, 102, 114, 133, 4, 152, 96, 109, 5, 31, 38, 162, 5, 104, 161, 31, 5, 122, 105, 249, 5, 251, 84, 250, 6, 79, 110, 137, 7, 3, 244, 28, 7, 9, 219, 117, 7, 181, 77, 64, 8, 7, 161, 231, 8, 145, 104, 163, 8, 173, 75, 162, 8, 200, 146, 64, 8, 225, 45, 186, 10, 150, 109, 26, 11, 61, 239, 86, 11, 66, 115, 186, 11, 108, 15, 214, 11, 175, 31, 42, 12, 103, 211, 11, 12, 144, 231, 249, 13, 20, 157, 60, 13, 180, 85, 66, 14, 14, 118, 138, 15, 9, 187, 119, 15, 176, 246, 126, 15, 210, 210, 39, 16, 13, 246, 129, 17, 154, 65, 65, 18, 5, 21, 239, 20, 45, 72, 60, 20, 54, 214, 254, 21, 218, 122, 179, 23, 198, 70, 254, 24, 221, 70, 64, 25, 186, 125, 170, 27, 120, 183, 23, 28, 234, 24, 143, 28, 248, 43, 42, 119, 254, 215, 84, 3, 7, 6, 230, 0, 0, 1, 104, 159, 123, 102, 206];
    messages::handle_message(10, &rlp::Rlp::new(&data));
}


pub fn main() {
    let mut buf = vec![0u8; 65535];
    let args: Vec<String> = std::env::args().collect();
    let node = aenode::Aenode::new(&args[1])
        //&String::from("aenode://pp_2kzKvxEg9NbBXn6krSeNec8kSeiJy8GXxnoTanX2zr1ffABvqd@192.168.111.81:3015"))
        .unwrap();
    let prologue = node.prologue(3013).unwrap();
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
    println!("Msg: {:?}", msg.as_raw());
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
