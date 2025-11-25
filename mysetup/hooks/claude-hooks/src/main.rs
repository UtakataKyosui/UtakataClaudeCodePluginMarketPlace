//! Production Claude Code Hooks
//! 
//! Comprehensive hook program for development workflow automation:
//! - Bash command logging and monitoring
//! - Rust file lint/format automation
//! - Information gathering result recording
//! - MCP tool usage tracking

use claude_code_hooks::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::Result;

use crate::session_stats::SessionStats;

mod session_stats;
mod bash_command;
mod rust_operation;
mod info_session;

#[tokio::main]
async fn main() -> Result<(), HookError> {
    // Initialize global state
    let stats = Arc::new(Mutex::new(SessionStats::default()));
    
    let hook = HookRunner::new()
        .on_pre_tool_use_sync({
            let stats = stats.clone();
            move |event| -> Result<(), HookError> {
                let mut session_stats = stats.lock().unwrap();
                
                // Initialize session if first event
                if session_stats.session_id.is_empty() {
                    *session_stats = SessionStats::new(event.common.session_id.clone());
                    println!("🚀 Starting session: {}", event.common.session_id);
                }
                
                let tool_name = &event.tool_name;
                let hook_event = HookEvent::PreToolUse(event.clone());
                
                // Handle different tool types
                match tool_name.as_str() {
                    "Bash" | "Task" => {
                        handle_bash_command(&mut session_stats, &event)?;
                    }
                    "Write" | "Edit" | "MultiEdit" => {
                        handle_file_operation(&mut session_stats, &event)?;
                    }
                    _ => {
                        // Check for MCP tools
                        if hook_event.is_mcp_tool() {
                            handle_mcp_tool(&mut session_stats, &hook_event)?;
                        } else {
                            println!("🔧 [Tool] {}", tool_name);
                        }
                    }
                }
                
                Ok(())
            }
        })
        .on_post_tool_use_sync({
            let stats = stats.clone();
            move |event| -> Result<(), HookError> {
                let mut session_stats = stats.lock().unwrap();
                let hook_event = HookEvent::PostToolUse(event.clone());
                
                // Handle post-tool processing
                if hook_event.is_mcp_tool() {
                    session_stats.update_info_session_result(&event);
                    
                    let category = helpers::mcp::categorize_mcp_tool(&hook_event);
                    if matches!(category, helpers::McpToolCategory::Documentation) {
                        println!("  📚 Information gathering completed");
                    }
                }
                
                // Handle Rust file operations completion
                if matches!(event.tool_name.as_str(), "Write" | "Edit" | "MultiEdit") {
                    if let Some(file_path) = event.tool_input.get("file_path") {
                        if let Some(path_str) = file_path.as_str() {
                            if path_str.ends_with(".rs") {
                                handle_rust_post_processing(&mut session_stats, &event, path_str)?;
                            }
                        }
                    }
                }
                
                Ok(())
            }
        })
        .on_stop_sync({
            let stats = stats.clone();
            move |event| -> Result<(), HookError> {
                let session_stats = stats.lock().unwrap();
                
                println!("🛑 Session ended: {}", event.common.session_id);
                session_stats.print_summary();
                
                // Save session data
                if let Err(e) = session_stats.save_to_file() {
                    eprintln!("Failed to save session data: {}", e);
                }
                
                Ok(())
            }
        });

    println!("🎯 Production Claude Code Hooks activated");
    println!("   Features: Bash logging, Rust automation, MCP tracking, Info recording");
    println!("   Logs saved to: ~/.claude/hook-logs/");
    println!("");
    
    hook.run_from_stdin().await
}

/// Handle Bash command execution
fn handle_bash_command(stats: &mut SessionStats, event: &PreToolUseEvent) -> Result<(), HookError> {
    if let Some(command) = event.tool_input.get("command") {
        if let Some(cmd_str) = command.as_str() {
            let is_destructive = stats.is_destructive_command(cmd_str);
            let is_system = stats.is_system_level_command(cmd_str);
            
            if is_destructive {
                println!("🚨 [BASH] ⚠️  DESTRUCTIVE: {}", cmd_str);
                
                // Send urgent notification
                if let Err(e) = helpers::logging::send_notification(
                    "🚨 Claude Code Warning",
                    &format!("Destructive command: {}", cmd_str.chars().take(50).collect::<String>())
                ) {
                    eprintln!("Failed to send notification: {}", e);
                }
            } else if is_system {
                println!("🔧 [BASH] System-level: {}", cmd_str);
            } else {
                println!("💻 [BASH] {}", cmd_str);
            }
            
            stats.record_bash_command(event);
        }
    }
    
    Ok(())
}

/// Handle file operations
fn handle_file_operation(stats: &mut SessionStats, event: &PreToolUseEvent) -> Result<(), HookError> {
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

/// Handle MCP tool usage
fn handle_mcp_tool(stats: &mut SessionStats, event: &HookEvent) -> Result<(), HookError> {
    if let Some(_info) = helpers::mcp::get_mcp_info(event) {
        let category = helpers::mcp::categorize_mcp_tool(event);
        let description = helpers::mcp::describe_mcp_usage(event);
        
        println!("🔌 [MCP] {}", description);
        
        // Special handling for different categories
        match category {
            helpers::McpToolCategory::Documentation => {
                println!("  📚 Gathering information...");
            }
            helpers::McpToolCategory::UiGeneration => {
                println!("  ✨ Generating UI component...");
            }
            helpers::McpToolCategory::BrowserAutomation => {
                println!("  🎭 Browser automation in progress...");
                if helpers::mcp::is_resource_intensive(event) {
                    println!("    ⏱️  This may take some time");
                }
            }
            helpers::McpToolCategory::Analysis => {
                println!("  🧠 Complex analysis running...");
            }
            helpers::McpToolCategory::Deployment => {
                println!("  🚀 Deployment operation...");
                if let Err(e) = helpers::logging::send_notification(
                    "Claude Code Deployment",
                    "Deployment operation started"
                ) {
                    eprintln!("Failed to send notification: {}", e);
                }
            }
            _ => {}
        }
        
        stats.record_mcp_usage(event);
    }
    
    Ok(())
}

/// Handle Rust file post-processing (lint and format)
fn handle_rust_post_processing(
    stats: &mut SessionStats, 
    event: &PostToolUseEvent, 
    file_path: &str
) -> Result<(), HookError> {
    let path = Path::new(file_path);
    
    // Check if it's a Rust file and if the operation was successful
    let success = event.tool_response.get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    if success && path.exists() {
        println!("  🔧 Running Rust automation for: {}", file_path);
        
        let mut lint_applied = false;
        let mut format_applied = false;
        
        // Try to run cargo fmt
        if let Ok(project_root) = find_cargo_project_root(path) {
            match std::process::Command::new("cargo")
                .arg("fmt")
                .arg("--")
                .arg(file_path)
                .current_dir(&project_root)
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("    ✨ Formatted with cargo fmt");
                        format_applied = true;
                    } else {
                        eprintln!("    ⚠️  cargo fmt failed: {}", 
                            String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => {
                    eprintln!("    ⚠️  Failed to run cargo fmt: {}", e);
                }
            }
            
            // Try to run cargo clippy on specific file
            match std::process::Command::new("cargo")
                .arg("clippy")
                .arg("--")
                .arg("-W")
                .arg("clippy::all")
                .current_dir(&project_root)
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("    🔍 Checked with cargo clippy");
                        lint_applied = true;
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if !stderr.contains("could not compile") {
                            println!("    🔍 Clippy analysis completed (with warnings)");
                            lint_applied = true;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("    ⚠️  Failed to run cargo clippy: {}", e);
                }
            }
        } else {
            // Try rustfmt directly
            match std::process::Command::new("rustfmt")
                .arg(file_path)
                .output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("    ✨ Formatted with rustfmt");
                        format_applied = true;
                    }
                }
                Err(_) => {
                    println!("    ℹ️  rustfmt not available");
                }
            }
        }
        
        // Update the last Rust operation record
        if let Some(rust_op) = stats.rust_operations.iter_mut()
            .filter(|r| r.file_path == file_path && r.session_id == event.common.session_id)
            .last() {
            rust_op.lint_applied = lint_applied;
            rust_op.format_applied = format_applied;
        }
    }
    
    Ok(())
}

/// Find the root directory of a Cargo project
fn find_cargo_project_root(file_path: &Path) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut current_dir = file_path.parent().unwrap_or(file_path);
    
    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(current_dir.to_path_buf());
        }
        
        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => return Err("No Cargo.toml found in parent directories".into()),
        }
    }
}