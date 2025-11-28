use anyhow::Result;
use claude_code_hooks::{HookError, PreToolUseEvent};

use crate::models::SessionStats;
use crate::security::{CommandValidator, SecurityLevel};
use crate::utils::{log_safely, send_notification_safely};

pub struct BashHandler {
    validator: CommandValidator,
}

impl Default for BashHandler {
    fn default() -> Self {
        Self {
            validator: CommandValidator::new(),
        }
    }
}

impl BashHandler {
    pub fn handle(&self, stats: &mut SessionStats, event: &PreToolUseEvent) -> Result<(), HookError> {
        if let Some(command) = event.tool_input.get("command") {
            if let Some(cmd_str) = command.as_str() {
                let validation = self.validator.validate(cmd_str);

                match validation.level {
                    SecurityLevel::Destructive => {
                        println!("🚨 [BASH] ⚠️  DESTRUCTIVE: {}", cmd_str);
                        send_notification_safely(
                            "🚨 Claude Code Warning",
                            "Destructive command detected",
                            &format!("Command: {}", cmd_str.chars().take(50).collect::<String>())
                        );
                        validation.print_warnings();
                    }
                    SecurityLevel::SystemLevel => {
                        println!("🔧 [BASH] System-level: {}", cmd_str);
                    }
                    SecurityLevel::Safe => {
                        println!("💻 [BASH] {}", cmd_str);
                    }
                }

                stats.record_bash_command(event);
                log_safely("BASH_COMMAND", cmd_str);
            }
        }

        Ok(())
    }
}
