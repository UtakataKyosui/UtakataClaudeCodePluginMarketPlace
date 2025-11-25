---
name: build-hooks
description: Builds the integrated Claude Code hooks in release mode. Use this skill when the plugin is first installed, when the hooks source code is updated, or when the user explicitly requests to rebuild the hooks. The skill compiles the Rust project located in hooks/claude-hooks/ and creates an optimized production binary.
---

# Build Hooks

Builds the integrated Claude Code hooks in release mode to create an optimized production binary.

## When to Use This Skill

- When the plugin is first installed
- After updating the hooks source code
- When the user explicitly requests to rebuild the hooks
- When troubleshooting hook issues that might be resolved by a clean build

## Instructions

1. **Navigate to the hooks directory**:
   ```bash
   cd ${CLAUDE_PLUGIN_ROOT}/hooks/claude-hooks/
   ```

2. **Run the release build**:
   ```bash
   cargo build --release
   ```

3. **Verify the build**:
   - Check that the binary was created at `target/release/integrated-claude-hooks`
   - Display the binary size using `ls -lh target/release/integrated-claude-hooks`
   - Confirm the build completed successfully with no errors

4. **Report the results**:
   - Show the binary path and size
   - Indicate successful completion
   - If the build fails, display the error and suggest solutions

## Expected Output

The release binary should be created at:
```
${CLAUDE_PLUGIN_ROOT}/hooks/claude-hooks/target/release/integrated-claude-hooks
```

Typical binary size: ~3.0MB (optimized release build)

## Features Included in the Hooks

The integrated hooks provide the following features:
- **Prompt Quality Evaluation**: Validates prompt quality before submission
- **Bash Command Security Validation**: Detects and warns about destructive commands
- **Multi-Language Auto-Format/Lint**: Supports Rust, Node.js, and Python
- **MCP Tool Tracking**: Monitors and categorizes MCP tool usage
- **Session Statistics**: Tracks session activity and saves logs to `~/.claude/hook-logs/`

## Troubleshooting

If the build fails, check:
- Rust and Cargo are installed: `cargo --version`
- All dependencies are available
- No syntax errors in the source code
- Sufficient disk space for the build

Common solutions:
- Update Rust toolchain: `rustup update`
- Clean build artifacts: `cargo clean` then retry
- Check error messages for missing dependencies
