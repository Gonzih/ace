use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub nats: Nats,
    pub naia: Naia,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Nats {
    pub host: String,
    pub port: usize,
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Naia {
    pub host: String,
    pub udp_port: usize,
    pub webrtc_port: usize,
    pub advertise_addr_udp: String,
    pub advertise_addr_webrtc: String,
}

pub fn parse(dir: &str, env: &str) -> Result<Config> {
    let path = Path::new(dir).join(format!("{}.yaml", env));
    let cfg = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&cfg)?;

    Ok(config)
}

pub fn include() -> Config {
    let cfg = include_str!(concat!("../../config/", env!("ACE_ENV"), ".yaml"));
    let config: Config = serde_yaml::from_str(&cfg).expect("Could not parse included config");

    config
}

impl Naia {
    pub fn advertise_addr(&self) -> String {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                self.advertise_addr_webrtc.clone()
            } else {
                self.advertise_addr_udp.clone()
            }
        }
    }
}
