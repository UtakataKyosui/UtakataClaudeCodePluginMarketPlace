use anyhow::Result;
use claude_code_hooks::{HookError, PostToolUseEvent, PreToolUseEvent};
use std::path::Path;

use crate::automation::{Formatter, Linter};
use crate::models::SessionStats;
use crate::utils::log_safely;

pub struct FileHandler;

impl FileHandler {
    pub fn handle_pre(&self, stats: &mut SessionStats, event: &PreToolUseEvent) -> Result<(), HookError> {
        if let Some(file_path) = event.tool_input.get("file_path") {
            if let Some(path_str) = file_path.as_str() {
                if path_str.ends_with(".rs") {
                    println!("🦀 [RUST] {} -> {}", event.tool_name, path_str);
                    stats.record_rust_operation(event);
                } else {
                    println!("📁 [FILE] {} -> {}", event.tool_name, path_str);
                }
            }
        }

        Ok(())
    }

    pub fn handle_post(&self, stats: &mut SessionStats, event: &PostToolUseEvent) -> Result<(), HookError> {
        if !matches!(event.tool_name.as_str(), "Write" | "Edit" | "MultiEdit") {
            return Ok(());
        }

        if let Some(file_path) = event.tool_input.get("file_path") {
            if let Some(path_str) = file_path.as_str() {
                let path = Path::new(path_str);

                if path_str.ends_with(".rs") && path.exists() {
                    println!("  🔧 Running Rust automation for: {}", path_str);

                    let format_results = Formatter::format_file(path);
                    let lint_results = Linter::lint_file(path);

                    let mut format_applied = false;
                    let mut lint_applied = false;

                    for result in &format_results {
                        result.print();
                        if matches!(result.status, crate::automation::FormatStatus::Success) {
                            format_applied = true;
                        }
                    }

                    for result in &lint_results {
                        result.print();
                        if matches!(result.status, crate::automation::LintStatus::Success | crate::automation::LintStatus::Warning) {
                            lint_applied = true;
                        }
                    }

                    // Update the last Rust operation record
                    if let Some(rust_op) = stats.rust_operations.iter_mut()
                        .filter(|r| r.file_path == path_str && r.session_id == event.common.session_id)
                        .last() {
                        rust_op.lint_applied = lint_applied;
                        rust_op.format_applied = format_applied;
                    }

                    log_safely("FILE_AUTOMATION", &format!("Rust file processed: {}", path_str));
                }
            }
        }

        Ok(())
    }
}
