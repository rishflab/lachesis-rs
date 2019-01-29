use ethkey::{Address, KeyPair, Secret, Public};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fs;
use std::io;
use std::error::Error;

use toml::{from_str};
use std::path::Path;
use hex;

#[derive(Debug, Clone, Deserialize)]
pub struct RawConfig {
    private_key:  String,
    address: Address,
    socket: SocketAddr,
    peers: Vec<Peer>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Peer {
    address: Address,
    socket: SocketAddr,
}

#[derive(Debug)]
pub struct Config {
    private_key: Secret,
    address: Address,
    socket: SocketAddr,
    peers: Vec<Peer>,
}

impl From<RawConfig> for Config {
    fn from(raw: RawConfig) -> Self {

        let s = hex::decode(raw.private_key).unwrap();

        Config {
            private_key: Secret::from_slice(s.as_slice()).unwrap(),
            address: raw.address,
            socket: raw.socket,
            peers: raw.peers,
        }
    }
}

fn load_config_from_file(path: &Path) -> Result<Config, Box<Error>> {

    let raw: RawConfig = toml::from_str(&fs::read_to_string(path)?)?;
    Ok(Config::from(raw))
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn parse_config_from_str() {
        let toml_str = r#"
        private_key = "9952830312ff9959fa15e438c82b2ebdd670e63035ff0531893027eaa49a8ca8"
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B60"
        socket = "127.0.0.1:8080"
        [[peers]]
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B61"
        socket = "127.0.0.1:8081"
        [[peers]]
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B62"
        socket = "127.0.0.1:8082"
    "#;

        let decoded: RawConfig = toml::from_str(toml_str).unwrap();
        let config = Config::from(decoded);
        println!("{:?}", config);
    }

    #[test]
    fn parse_config_from_file() {

        let config = load_config_from_file(Path::new("res/node1.toml")).unwrap();

        println!("{:?}", config);

    }
}



