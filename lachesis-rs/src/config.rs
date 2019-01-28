use ethkey::{Address, KeyPair};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use toml::{from_str};

#[derive(Debug, Deserialize)]
pub struct Config {
    address: Address,
    socket: SocketAddr,
    peers: Vec<Peer>,
}

#[derive(Debug, Deserialize)]
pub struct Peer {
    address: Address,
    socket: SocketAddr,
}


//Convert to these structs
//#[derive(Debug, Deserialize)]
//pub struct Config {
//    peers: Vec<Peer>,
//    keypair: KeyPair,
//    address: Address,
//    socket: SocketAddr,
//}
//
//#[derive(Debug, Deserialize)]
//pub struct Peer {
//    address: Address,
//    socket: SocketAddr,
//}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn parse_config_from_str() {
        let toml_str = r#"
        keypair = "Wefwefwef"
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B60"
        socket = "127.0.0.1:8080"
        [[peers]]
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B61"
        socket = "127.0.0.1:8081"
        [[peers]]
        address = "0xB1CCDB544f603Af631525Ec406245909ad6e1B62"
        socket = "127.0.0.1:8082"
    "#;

        let decoded: Config = toml::from_str(toml_str).unwrap();
        println!("{:#?}", decoded);
    }
}



