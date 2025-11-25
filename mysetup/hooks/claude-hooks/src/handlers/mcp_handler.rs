use anyhow::Result;
use claude_code_hooks::{helpers, HookError, HookEvent, PostToolUseEvent};

use crate::models::SessionStats;
use crate::utils::{log_safely, send_notification_safely};

pub struct McpHandler;

impl McpHandler {
    pub fn handle_pre(&self, stats: &mut SessionStats, event: &HookEvent) -> Result<(), HookError> {
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
                    send_notification_safely(
                        "Claude Code Deployment",
                        "Deployment operation",
                        "Deployment operation started"
                    );
                }
                _ => {}
            }

            stats.record_mcp_usage(event);
            log_safely("MCP_TOOL", &description);
        }

        Ok(())
    }

    pub fn handle_post(&self, stats: &mut SessionStats, event: &PostToolUseEvent) -> Result<(), HookError> {
        let hook_event = HookEvent::PostToolUse(event.clone());

        if hook_event.is_mcp_tool() {
            stats.update_info_session_result(event);

            let category = helpers::mcp::categorize_mcp_tool(&hook_event);
            if matches!(category, helpers::McpToolCategory::Documentation) {
                println!("  📚 Information gathering completed");
            }
        }

        Ok(())
    }
}
