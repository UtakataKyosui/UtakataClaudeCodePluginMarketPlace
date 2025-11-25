/// 危険なコマンドかどうかを判定する関数
pub fn is_dangerous_command(tool_name: &str, input: &str) -> bool {
    let dangerous_patterns = [
        r"rm\s+-rf",
        r"sudo\s+rm",
        r"chmod\s+777",
        r"curl.*\|\s*bash",
        r"wget.*\|\s*bash",
        r"dd\s+if=",
        r"mkfs\.",
        r"format\s+c:",
        r"del\s+/s\s+/q",
        r"shutdown",
        r"reboot",
        r"halt",
    ];

    for pattern in dangerous_patterns.iter() {
        if regex::Regex::new(pattern).unwrap().is_match(input) {
            return true;
        }
    }

    // 危険なツール名もチェック
    match tool_name {
        "Bash" | "bash" | "shell" => dangerous_patterns
            .iter()
            .any(|pattern| regex::Regex::new(pattern).unwrap().is_match(input)),
        _ => false,
    }
}



