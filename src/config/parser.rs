use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
    pub target_port: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoAuthEndpoints {
    pub endpoint: String,
    pub method: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub api_gateway_url: String,
    pub is_https: bool,
    pub authorization_api_url: String,
    pub services: Vec<ServiceConfig>,
    pub endpoints_without_auth: Vec<NoAuthEndpoints>,
    pub logger_config: LoggerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggerConfig {
    pub use_kafka: bool,
    pub kafka_host: Option<String>,
    pub kafka_topic: Option<String>,
    pub out_file: String,
    pub err_file: String,
    pub debug_file: String,
}

pub fn load_config(config_path: &str) -> GatewayConfig {
    let mut file = File::open(config_path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read config file");

    serde_yaml::from_str(&contents).expect("Error parsing config file")
}
