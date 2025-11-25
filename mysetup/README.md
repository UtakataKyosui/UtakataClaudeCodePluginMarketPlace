# Utakata Claude Code Vibe-Coding Plugin

Utakata Kyosui's Claude Code plugin with integrated hooks for enhanced development experience.

## Features

### Integrated Hooks System
- 🎯 **Prompt Quality Evaluation**: Automatically evaluates prompt quality and blocks low-quality prompts
- 🛡️ **Bash Command Security Validation**: Detects and warns about destructive commands
- 🔧 **Multi-Language Auto-Format/Lint**: Supports Rust, Node.js, and Python projects
- 📊 **MCP Tool Tracking**: Monitors and categorizes MCP tool usage
- 📈 **Session Statistics**: Tracks session activity and saves logs

## Installation

### Prerequisites
- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))
- Claude Code CLI

### Setup

1. Install the plugin through Claude Code plugin manager, or clone manually:
   ```bash
   git clone https://github.com/UtakataKyosui/UtakataClaudePlugin.git
   ```

2. Run the setup script:
   ```bash
   cd mysetup
   ./setup.sh
   ```

   The setup script will:
   - Check for Rust/Cargo installation
   - Build the integrated hooks in release mode
   - Display build status and binary information

## Agent Skills

This plugin includes three agent skills for managing the hooks system:

### 1. build-hooks

Builds the integrated hooks in release mode.

**When to use:**
- After plugin installation
- When hooks source code is updated
- When explicitly rebuilding hooks

**Usage in Claude Code:**
```
Use the "build-hooks" skill
```

**What it does:**
1. Navigates to the hooks project directory
2. Runs `cargo build --release`
3. Verifies the binary was created successfully
4. Displays build information

### 2. check-hook-logs

Displays and analyzes hooks logs for troubleshooting.

**When to use:**
- Troubleshooting prompt blocking or allowance
- Reviewing bash command security warnings
- Analyzing MCP tool usage patterns
- Investigating unexpected hook behavior

**Usage in Claude Code:**
```
Use the "check-hook-logs" skill
```

**What it does:**
1. Lists recent log files in `~/.claude/hook-logs/`
2. Displays recent log entries
3. Searches for specific events (prompts, security, MCP tools)
4. Analyzes and reports findings

### 3. clean-build-hooks

Performs a clean build by removing all build artifacts first.

**When to use:**
- Encountering unexplained build errors
- After updating Rust toolchain
- When build artifacts may be corrupted
- Troubleshooting build issues

**Usage in Claude Code:**
```
Use the "clean-build-hooks" skill
```

**What it does:**
1. Runs `cargo clean` to remove all build artifacts
2. Performs fresh `cargo build --release`
3. Verifies the new binary
4. Reports results

**Manual operations:**
```bash
# Regular build
cd hooks/claude-hooks
cargo build --release

# Clean build
cd hooks/claude-hooks
cargo clean && cargo build --release

# Check logs
ls -lht ~/.claude/hook-logs/
tail -50 ~/.claude/hook-logs/*.log
```

## Project Structure

```
mysetup/
├── .claude-plugin/
│   └── plugin.json          # Plugin metadata
├── hooks/
│   ├── claude-hooks/        # Integrated hooks source code
│   │   ├── src/
│   │   │   ├── models/      # Data structures
│   │   │   ├── handlers/    # Event handlers
│   │   │   ├── automation/  # Format/lint automation
│   │   │   ├── security/    # Security validation
│   │   │   └── utils/       # Logging and notifications
│   │   ├── build.rs         # Build script for macOS frameworks
│   │   ├── Cargo.toml       # Rust project configuration
│   │   └── target/release/
│   │       └── integrated-claude-hooks  # Built binary
│   └── hooks.json           # Hook configuration
├── skills/                  # Agent Skills
│   ├── build-hooks/         # Build hooks skill
│   │   └── SKILL.md
│   ├── check-hook-logs/     # Check logs skill
│   │   └── SKILL.md
│   └── clean-build-hooks/   # Clean build skill
│       └── SKILL.md
├── setup.sh                 # Setup script
└── README.md               # This file
```

## Hook Events

The plugin registers hooks for the following events:

- **UserPromptSubmit**: Evaluates prompt quality before submission
- **PreToolUse**: Validates commands and tracks MCP tool usage
- **PostToolUse**: Logs tool usage and triggers automation (format/lint)
- **Stop**: Saves session statistics when Claude Code session ends

## Logs

All hook activities are logged to:
```
~/.claude/hook-logs/
```

## Development

### Building from Source

```bash
cd hooks/claude-hooks
cargo build --release
```

### Running Tests

```bash
cd hooks/claude-hooks
cargo test
```

### Updating Dependencies

```bash
cd hooks/claude-hooks
cargo update
```

## Troubleshooting

### Build Fails

If the build fails, ensure you have:
1. Latest Rust toolchain: `rustup update`
2. Required system libraries for macOS notifications (AppKit, Foundation, etc.)

### Hooks Not Working

1. Verify the binary exists:
   ```bash
   ls -lh hooks/claude-hooks/target/release/integrated-claude-hooks
   ```

2. Check hook configuration:
   ```bash
   cat hooks/hooks.json
   ```

3. Review logs:
   ```bash
   ls -la ~/.claude/hook-logs/
   ```

## License

MIT License - see repository for details

## Author

**Utakata Kyosui**
- Email: fill.ayaextech@gmail.com
- GitHub: [UtakataKyosui](https://github.com/UtakataKyosui)

## Repository

https://github.com/UtakataKyosui/UtakataClaudePlugin.git
