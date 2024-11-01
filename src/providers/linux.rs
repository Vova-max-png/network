use std::process::Command;
use crate::providers::NetworkBehavior;

use crate::{types::{EncryptionType, Error, NetworkType}};

#[derive(Debug)]
pub struct Net {
    ssid: Box<str>,
    typ: NetworkType,
    encryption: EncryptionType,
}

pub struct Network {
    available_networks: Vec<Net>,
}

// TODO: make functions work on linux
impl NetworkBehavior for Network {
    fn new() -> Self {
        Self {
            available_networks: Vec::new()
        }
    }

    fn get_around(&self) -> Result<Self, Error> {
        let res = Command::new("netsh")
            .args(["wlan", "show", "networks"])
            .output();

        match(res) {
            Err(e) => {return Err(Error::GettingAroundFailed)},
            _ => {}
        }

        let binding = &String::from_utf8_lossy(&res.unwrap().stdout).to_string().replace(" ", "");
        let mut out: Vec<&str> = binding.split("\r\n\r\n").collect();
        out.remove(0);
        out.remove(out.len()-1);

        let mut ssids: Vec<&str> = Vec::new();
        let mut auth_types: Vec<&str> = Vec::new();
        let mut network_types: Vec<&str> = Vec::new();

        /*Getting SSIDS, network types, auth types from the command result*/
        for i in &out {
            let info: Vec<&str> = i.split("\r\n").collect();
            let ssid: Vec<&str> = info[0].split(":").collect();
            ssids.push(ssid.get(1).unwrap());
            let network_type: Vec<&str> = info[1].split(":").collect();
            network_types.push(network_type.get(1).unwrap());
            let auth_type: Vec<&str> = info[2].split(":").collect();
            auth_types.push(auth_type.get(1).unwrap());
        }

        let mut nets: Vec<Net> = Vec::new();
        for i in 0..out.len() {
            let network_type = match(network_types[i]) {
                "Infrastructure" => NetworkType::Infrastructure,
                _ => NetworkType::Unknown
            };

            let auth_type = match(auth_types[i]) {
                "WPA2-Personal" => EncryptionType::Wpa2,
                _ => EncryptionType::Unknown
            };

            nets.push(Net {
                ssid: ssids[i].into(),
                typ: network_type,
                encryption: auth_type
            });
        }

        Ok(Self { 
            available_networks: nets
        })
    }

    fn print_around(&self) -> Result<&Vec<Net>, Error> {
        Ok(&self.available_networks)
    }

    fn connect(&self, ssid: &str, pass: &str) {
        let res = Command::new("netsh")
            .args(["wlan", "set", "hostednetwork", "mode=allow", format!("ssid={}", ssid).as_str(), format!("key={}", pass).as_str()])
            .output();

        println!("{:#?}", res);
    }
}

impl Net {
    pub fn get_ssid(&self) -> &str {
        &self.ssid
    }

    pub fn get_network_type(&self) -> &NetworkType {
        &self.typ
    }

    pub fn get_encryption_type(&self) -> &EncryptionType {
        &self.encryption
    }
}