use serde::Deserialize;
use std::fs;

pub fn load_config(path: &str) -> Result<BatchConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: BatchConfig = toml::from_str(&content)?;
    Ok(config)
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Copy,
    Move,
    Delete,
}

#[derive(Deserialize)]
pub struct Operation {
    pub op_type: OperationType,

    pub from: Option<String>,
    pub to: Option<String>,
    pub target: Option<String>,

    pub recursive: Option<bool>,
    pub age_hours: Option<u64>,
    pub interval: Option<u64>,
}

#[derive(Deserialize)]
pub struct BatchConfig {
    pub operations: Vec<Operation>,
}
