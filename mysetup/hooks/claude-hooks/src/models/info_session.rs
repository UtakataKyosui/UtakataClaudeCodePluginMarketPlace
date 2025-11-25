use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Information gathering session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoSession {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub tool_name: String,
    pub query_info: serde_json::Value,
    pub result_summary: Option<String>,
}