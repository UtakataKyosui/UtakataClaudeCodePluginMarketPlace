use std::fs;

use chrono::Utc;
use dirs::home_dir;

/// ログファイルに記録する関数
pub fn log_to_file(event_type: &str, details: &str) {
    if let Some(home) = home_dir() {
        let log_dir = home.join(".claude").join("hook-logs");
        if fs::create_dir_all(&log_dir).is_err() {
            return;
        }

        let log_file = log_dir.join("hook.log");
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let log_entry = format!("[{timestamp}] {event_type}: {details}\n");

        let _ = fs::write(
            &log_file,
            format!(
                "{}{}",
                fs::read_to_string(&log_file).unwrap_or_default(),
                log_entry
            ),
        );
    }
}