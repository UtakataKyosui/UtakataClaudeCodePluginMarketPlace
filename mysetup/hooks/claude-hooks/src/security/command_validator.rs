use regex::Regex;

/// Command security validator
pub struct CommandValidator {
    destructive_patterns: Vec<Regex>,
    system_level_patterns: Vec<Regex>,
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandValidator {
    pub fn new() -> Self {
        let destructive_patterns = vec![
            r"rm\s+-rf",
            r"rm\s+-r",
            r"sudo\s+rm",
            r"rm\s+/",
            r"del\s+",
            r"rmdir",
            r"format\s+",
            r"fdisk",
            r"dd\s+if=",
            r"sudo\s+dd",
            r">\s*/dev/",
            r"sudo\s+chmod\s+777",
            r"chmod\s+-R\s+777",
            r"sudo\s+chown\s+-R",
            r"curl.*\|\s*bash",
            r"wget.*\|\s*bash",
            r"mkfs\.",
            r"format\s+c:",
            r"del\s+/s\s+/q",
            r"shutdown",
            r"reboot",
            r"halt",
        ]
        .into_iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

        let system_level_patterns = vec![
            r"sudo\s+",
            r"su\s+",
            r"doas\s+",
            r"systemctl",
            r"/etc/",
            r"/var/",
            r"/usr/",
            r"/bin/",
            r"/sbin/",
            r"mount\s+",
            r"umount\s+",
            r"iptables",
            r"firewall",
            r"passwd",
            r"useradd",
            r"userdel",
        ]
        .into_iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

        Self {
            destructive_patterns,
            system_level_patterns,
        }
    }

    /// Check if a command is destructive
    pub fn is_destructive(&self, command: &str) -> bool {
        self.destructive_patterns
            .iter()
            .any(|pattern| pattern.is_match(command))
    }

    /// Check if a command is system-level
    pub fn is_system_level(&self, command: &str) -> bool {
        self.system_level_patterns
            .iter()
            .any(|pattern| pattern.is_match(command))
    }

    /// Get security level of a command
    pub fn get_security_level(&self, command: &str) -> SecurityLevel {
        if self.is_destructive(command) {
            SecurityLevel::Destructive
        } else if self.is_system_level(command) {
            SecurityLevel::SystemLevel
        } else {
            SecurityLevel::Safe
        }
    }

    /// Validate command with detailed information
    pub fn validate(&self, command: &str) -> CommandValidation {
        let is_destructive = self.is_destructive(command);
        let is_system_level = self.is_system_level(command);

        let level = if is_destructive {
            SecurityLevel::Destructive
        } else if is_system_level {
            SecurityLevel::SystemLevel
        } else {
            SecurityLevel::Safe
        };

        let warnings = self.generate_warnings(command, is_destructive, is_system_level);

        CommandValidation {
            command: command.to_string(),
            level,
            is_destructive,
            is_system_level,
            warnings,
        }
    }

    fn generate_warnings(
        &self,
        command: &str,
        is_destructive: bool,
        is_system_level: bool,
    ) -> Vec<String> {
        let mut warnings = Vec::new();

        if is_destructive {
            warnings.push("⚠️  This command may cause irreversible data loss".to_string());

            if command.contains("rm -rf /") || command.contains("rm -r /") {
                warnings.push("🚨 CRITICAL: This command targets the root directory!".to_string());
            }

            if command.contains("sudo") {
                warnings.push("⚠️  Running with elevated privileges".to_string());
            }
        }

        if is_system_level {
            warnings.push("🔧 This command operates at the system level".to_string());

            if command.contains("/etc/") {
                warnings.push("⚠️  Modifying system configuration files".to_string());
            }
        }

        if command.contains("|") && (command.contains("bash") || command.contains("sh")) {
            warnings.push("⚠️  Piping to shell - potential security risk".to_string());
        }

        warnings
    }
}

/// Security level of a command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Safe,
    SystemLevel,
    Destructive,
}

impl SecurityLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            SecurityLevel::Safe => "✅",
            SecurityLevel::SystemLevel => "🔧",
            SecurityLevel::Destructive => "🚨",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SecurityLevel::Safe => "Safe",
            SecurityLevel::SystemLevel => "System-level",
            SecurityLevel::Destructive => "Destructive",
        }
    }
}

/// Command validation result
#[derive(Debug, Clone)]
pub struct CommandValidation {
    pub command: String,
    pub level: SecurityLevel,
    pub is_destructive: bool,
    pub is_system_level: bool,
    pub warnings: Vec<String>,
}

impl CommandValidation {
    pub fn print_warnings(&self) {
        if !self.warnings.is_empty() {
            println!("{} Command Security Analysis:", self.level.emoji());
            for warning in &self.warnings {
                println!("  {}", warning);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_command() {
        let validator = CommandValidator::new();
        assert_eq!(
            validator.get_security_level("ls -la"),
            SecurityLevel::Safe
        );
    }

    #[test]
    fn test_system_command() {
        let validator = CommandValidator::new();
        assert_eq!(
            validator.get_security_level("sudo systemctl restart nginx"),
            SecurityLevel::SystemLevel
        );
    }

    #[test]
    fn test_destructive_command() {
        let validator = CommandValidator::new();
        assert_eq!(
            validator.get_security_level("rm -rf /tmp/test"),
            SecurityLevel::Destructive
        );
    }
}
