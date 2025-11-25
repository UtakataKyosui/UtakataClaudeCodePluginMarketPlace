#[cfg(target_os = "macos")]
use mac_notification_sys::{get_bundle_identifier_or_default, send_notification};

/// macOS通知を送信する関数
#[cfg(target_os = "macos")]
pub fn send_mac_notification(title: &str, subtitle: &str, message: &str) {
    let _bundle = get_bundle_identifier_or_default("Claude Code Hook");
    let _ = send_notification(title, Some(subtitle), message, None);
}

/// macOS以外の環境では何もしない
#[cfg(not(target_os = "macos"))]
pub fn send_mac_notification(_title: &str, _subtitle: &str, _message: &str) {
    // No-op for non-macOS platforms
}