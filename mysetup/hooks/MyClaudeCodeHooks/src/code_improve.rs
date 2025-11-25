use std::{path::Path, process::Command};

use crate::{logging::log_to_file, mac_notification::send_mac_notification};

/// 自動フォーマット・リントを実行する関数
pub fn run_format_and_lint() -> Result<(), String> {
    let mut results = Vec::new();

    // Rust プロジェクトの場合
    if Path::new("Cargo.toml").exists() {
        // cargo fmt を実行
        match Command::new("cargo").args(["fmt"]).output() {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ cargo fmt 完了".to_string());
                } else {
                    results.push(format!(
                        "❌ cargo fmt エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(e) => results.push(format!("❌ cargo fmt 実行エラー: {e}")),
        }

        // cargo clippy を実行
        match Command::new("cargo")
            .args(["clippy", "--", "-D", "warnings"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ cargo clippy 完了".to_string());
                } else {
                    results.push(format!(
                        "⚠️ cargo clippy 警告/エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(e) => results.push(format!("❌ cargo clippy 実行エラー: {e}")),
        }
    }

    // Node.js プロジェクトの場合
    if Path::new("package.json").exists() {
        // prettier を実行
        match Command::new("npx")
            .args(["prettier", "--write", "."])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ prettier フォーマット完了".to_string());
                } else {
                    results.push(format!(
                        "❌ prettier エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(_) => {
                // prettier がない場合は npm run format を試す
                match Command::new("npm").args(["run", "format"]).output() {
                    Ok(output) => {
                        if output.status.success() {
                            results.push("✅ npm run format 完了".to_string());
                        }
                    }
                    Err(_) => results
                        .push("⚠️ prettier または npm run format が見つかりません".to_string()),
                }
            }
        }

        // eslint を実行
        match Command::new("npx").args(["eslint", ".", "--fix"]).output() {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ eslint 完了".to_string());
                } else {
                    results.push(format!(
                        "⚠️ eslint 警告/エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(_) => {
                // eslint がない場合は npm run lint を試す
                match Command::new("npm").args(["run", "lint"]).output() {
                    Ok(output) => {
                        if output.status.success() {
                            results.push("✅ npm run lint 完了".to_string());
                        }
                    }
                    Err(_) => {
                        results.push("⚠️ eslint または npm run lint が見つかりません".to_string())
                    }
                }
            }
        }
    }

    // Python プロジェクトの場合
    if Path::new("requirements.txt").exists() || Path::new("pyproject.toml").exists() {
        // black を実行
        match Command::new("black").args(["."]).output() {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ black フォーマット完了".to_string());
                } else {
                    results.push(format!(
                        "❌ black エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(_) => results.push("⚠️ black が見つかりません".to_string()),
        }

        // flake8 を実行
        match Command::new("flake8").args(["."]).output() {
            Ok(output) => {
                if output.status.success() {
                    results.push("✅ flake8 完了".to_string());
                } else {
                    results.push(format!(
                        "⚠️ flake8 警告/エラー: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(_) => results.push("⚠️ flake8 が見つかりません".to_string()),
        }
    }

    let summary = results.join("\n");

    // 通知を送信
    send_mac_notification("Claude Code - フォーマット/リント完了", "", &summary);

    // ログに記録
    log_to_file("FORMAT_LINT", &summary);

    if results.iter().any(|r| r.starts_with("❌")) {
        Err(summary)
    } else {
        Ok(())
    }
}