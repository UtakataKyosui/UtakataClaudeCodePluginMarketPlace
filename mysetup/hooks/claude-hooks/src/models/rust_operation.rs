use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Rust file operation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustOperation {
    pub timestamp: DateTime<Utc>,
    pub operation_type: String,
    pub file_path: String,
    pub session_id: String,
    pub lint_applied: bool,
    pub format_applied: bool,
}