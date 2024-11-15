use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
    pub target_port: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub api_gateway_url: String,
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_yaml::from_str(&contents).unwrap()
}
