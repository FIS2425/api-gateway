use openapiv3::{OpenAPI, Server};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct OpenApiMerger {
    url: String,
    base_path: String,
    specs: HashMap<String, OpenAPI>,
    output_path: String,
}

impl OpenApiMerger {
    pub fn new(url: &str, docs_path: &str, output_path: &str) -> Self {
        OpenApiMerger {
            url: url.to_string(),
            base_path: docs_path.to_string(),
            specs: HashMap::new(),
            output_path: output_path.to_string(),
        }
    }

    pub fn load_specs(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        for entry in WalkDir::new(&self.base_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "yaml" || e == "yml") {
                let content = fs::read_to_string(path)?;
                let spec: OpenAPI = serde_yaml::from_str(&content)?;
                let service_name = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                self.specs.insert(service_name, spec);
            }
        }
        Ok(())
    }

    pub fn merge(&self) -> Result<OpenAPI, Box<dyn Error + Send + Sync>> {
        let mut merged_spec = OpenAPI {
            openapi: "3.0.0".to_string(),
            info: openapiv3::Info {
                title: "API Gateway Documentation".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Automatically merged API documentation".to_string()),
                ..Default::default()
            },
            servers: vec![Server {
                url: format!("http://{}", self.url),
                ..Default::default()
            }],
            ..Default::default()
        };

        for (_service, spec) in &self.specs {
            for (path, path_item) in spec.paths.iter() {
                merged_spec.paths.paths.insert(
                    path.clone(),
                    path_item.clone(),
                );
            }

            if let Some(components) = &spec.components {
                let merged_components = merged_spec.components.get_or_insert(Default::default());
                for (name, schema) in &components.schemas {
                    merged_components.schemas.insert(name.clone(), schema.clone());
                }
                for (name, scheme) in &components.security_schemes {
                    merged_components.security_schemes.insert(name.clone(), scheme.clone());
                }
            }
        }

        let yaml = serde_yaml::to_string(&merged_spec)?;
        fs::write(&self.output_path, yaml)?;
        Ok(merged_spec)
    }

    pub fn load(&self) -> Result<OpenAPI, Box<dyn Error + Send + Sync>> {
        let content = fs::read_to_string(&self.output_path)?;
        let spec: OpenAPI = serde_yaml::from_str(&content)?;
        Ok(spec)
    }
}
