use super::parser::LoggerConfig;
use chrono::{SecondsFormat, Utc};
use kafka::producer::{Producer, Record, RequiredAcks};
use serde_json::json;
use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Duration;

pub struct Logger {
    use_kafka: bool,
    kafka_host: Option<String>,
    kafka_topic: String,
    out_file: String,
    err_file: String,
}

impl Logger {
    pub fn from_config(config: &LoggerConfig) -> Logger {
        // Create dir if it doesn't exist
        if !Path::new(&config.out_file).exists() {
            create_dir_all(Path::new(&config.out_file).parent().unwrap()).unwrap();
        }
        if !Path::new(&config.err_file).exists() {
            create_dir_all(Path::new(&config.err_file).parent().unwrap()).unwrap();
        }

        if !config.use_kafka {
            Logger {
                use_kafka: false,
                kafka_host: None,
                kafka_topic: "".to_string(),
                out_file: config.out_file.clone(),
                err_file: config.err_file.clone(),
            }
        } else {
            Logger {
                use_kafka: true,
                kafka_host: config.kafka_host.clone(),
                kafka_topic: config.kafka_topic.clone().unwrap_or("".to_string()),
                out_file: config.out_file.clone(),
                err_file: config.err_file.clone(),
            }
        }
    }

    pub fn info(&self, message: &str, params: &[(&str, &str)]) {
        let log_entry = self.build_log_entry("info", message, params);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.out_file)
            .unwrap();

        Write::write_fmt(&mut file, format_args!("{}\n", log_entry)).unwrap();

        if self.use_kafka {
            self.send_to_kafka(&log_entry);
        }
    }

    pub fn warn(&self, message: &str, params: &[(&str, &str)]) {
        let log_entry = self.build_log_entry("warn", message, params);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.out_file)
            .unwrap();

        Write::write_fmt(&mut file, format_args!("{}\n", log_entry)).unwrap();

        if self.use_kafka {
            self.send_to_kafka(&log_entry);
        }
    }

    pub fn err(&self, message: &str, params: &[(&str, &str)]) {
        let log_entry = self.build_log_entry("err", message, params);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.err_file)
            .unwrap();

        Write::write_fmt(&mut file, format_args!("{}\n", log_entry)).unwrap();

        if self.use_kafka {
            self.send_to_kafka(&log_entry);
        }
    }

    fn build_log_entry(&self, level: &str, message: &str, params: &[(&str, &str)]) -> String {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

        let mut log_obj = json!({
            "service": "api-gateway",
            "timestamp": timestamp,
            "level": level,
            "message": message,
        });

        let additional_params: HashMap<_, _> = params.iter().cloned().collect();

        if !additional_params.is_empty() {
            log_obj["params"] = json!(additional_params);
        }

        serde_json::to_string(&log_obj).unwrap()
    }

    fn send_to_kafka(&self, message: &str) {
        let kafka_host = match &self.kafka_host {
            Some(host) => host,
            None => return,
        };
        let mut producer = match Producer::from_hosts(vec![kafka_host.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
        {
            Ok(producer) => Some(producer),
            Err(_) => None,
        };
        if let Some(producer) = &mut producer {
            producer
                .send(&Record::from_value(&self.kafka_topic, message.as_bytes()))
                .unwrap();
        }
    }
}
