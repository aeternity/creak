use regex::Regex;
use std::net::IpAddr;

pub struct Aenode {
    pub pub_key: String,
    pub address: IpAddr,
    pub port: u16,
}

impl Aenode {
    pub fn new(aenode: &String) -> Result<Aenode, Box<std::error::Error>> {
        let re = Regex::new(r"^aenode://pp_(.+)@([0-9.]+):([0-9]+)$")?;
        let captures = re.captures(aenode).unwrap();
        Ok(Aenode {
            pub_key: captures[1].parse()?,
            address: captures[2].parse()?,
            port: captures[3].parse()?,
        })
    }
}


