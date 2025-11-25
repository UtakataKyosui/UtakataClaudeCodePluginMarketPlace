use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Bash command execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashCommand {
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub session_id: String,
    pub cwd: String,
    pub is_destructive: bool,
    pub is_system_level: bool,
}
