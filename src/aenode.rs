use base58check::FromBase58Check;
use byteorder::{BigEndian, WriteBytesExt};
use curl::easy::Easy;
use regex::Regex;
use serde_json::Value;
use simple_error::SimpleError;
use std::fmt::{Error, Write};
use std::net::IpAddr;

type RlpError = Box<std::error::Error>;

pub struct Aenode {
    pub pub_key: String,
    pub address: IpAddr,
    pub port: u16,
}

impl Aenode {
    pub fn new(aenode: &String) -> Result<Aenode, Box<std::error::Error>>
    {
        let re = Regex::new(r"^aenode://pp_(.+)@([0-9.]+):([0-9]+)$")?;
        let captures = re.captures(aenode).unwrap();
        Ok(Aenode {
            pub_key: captures[1].parse()?,
            address: captures[2].parse()?,
            port: captures[3].parse()?,
        })
    }

    pub fn prologue(&self, port: u16) -> Result<Vec<u8>, RlpError>
    {
        let mut easy = Easy::new();
        let url = format!("http://{}:{}/v2/status", self.address, port);
        println!("{}", url);
        easy.url(&url)?;
        let mut v = Vec::<u8>::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                v.extend_from_slice(data);
                Ok(data.len())
            });
            transfer.perform();
        }
        let json: Value = serde_json::from_str(&String::from_utf8(v)?)?;
        println!("{:?}", json);
        let gen_hash = match json["genesis_key_block_hash"].as_str() {
            Some(x) => String::from(x),
            None => bail!("Genesis block not found"),
        };
        let gen_hash = String::from_utf8(gen_hash[3..].as_bytes().to_vec())?;
        let network_id = match json["network_id"].as_str() {
            Some(x) => String::from(x),
            None => bail!("Network id not found"),
        };
        prologue(1, &gen_hash, &network_id)
    }
}

/*
 * decode base 58, adding the version byte onto the returned value
 */
fn decodebase58check(data: &String) -> Vec<u8>
{
    let mut result = base58check::FromBase58Check::from_base58check(data.as_str()).unwrap().1;
    result.insert(0, base58check::FromBase58Check::from_base58check(data.as_str()).unwrap().0);
    result
}



pub fn prologue(version: u64, genesis_hash: &String, network_id: &String) ->
    Result<Vec<u8>, RlpError>
{
    let mut genesis_binary = decodebase58check(&genesis_hash);
    let mut network_id_binary = network_id.as_bytes();
    let mut result = vec!();
    result.write_u64::<BigEndian>(version);
    result.append(&mut genesis_binary);
    result.append(&mut network_id_binary.to_vec());
    Ok(result)
}
