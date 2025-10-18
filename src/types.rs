use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateResponse {
    pub feature_key: String,
    pub enabled: bool,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub server_time: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureHealth {
    pub feature_key: String,
    pub environment_key: String,
    pub enabled: bool,
    pub auto_disabled: bool,
    pub error_rate: Option<f32>,
    pub threshold: Option<f32>,
    pub last_error_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureErrorReport {
    pub error_type: String,
    pub error_message: String,
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
}
