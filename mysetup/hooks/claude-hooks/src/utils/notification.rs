use anyhow::Result;

/// Send macOS notification
#[cfg(target_os = "macos")]
pub fn send_notification(title: &str, subtitle: &str, message: &str) -> Result<()> {
    use mac_notification_sys::{get_bundle_identifier_or_default, send_notification as send_mac};

    let _bundle = get_bundle_identifier_or_default("Claude Code Hook");
    send_mac(title, Some(subtitle), message, None)
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("Failed to send notification: {}", e))
}

/// Non-macOS platforms: no-op
#[cfg(not(target_os = "macos"))]
pub fn send_notification(_title: &str, _subtitle: &str, _message: &str) -> Result<()> {
    Ok(())
}

/// Send notification without propagating errors
pub fn send_notification_safely(title: &str, subtitle: &str, message: &str) {
    if let Err(e) = send_notification(title, subtitle, message) {
        eprintln!("Failed to send notification: {}", e);
    }
}

/// Macro for sending notifications with formatted messages
#[macro_export]
macro_rules! notify {
    ($title:expr, $subtitle:expr, $($arg:tt)*) => {
        $crate::utils::notification::send_notification_safely(
            $title,
            $subtitle,
            &format!($($arg)*)
        )
    };
}
