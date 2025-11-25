#!/bin/bash
# Setup script for Utakata Claude Code Plugin
# This script builds the integrated hooks in release mode

set -e

echo "🔧 Setting up Utakata Claude Code Plugin..."

# Get the plugin root directory
PLUGIN_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOKS_DIR="${PLUGIN_ROOT}/hooks/claude-hooks"

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed"
    echo "   Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "📦 Building integrated-claude-hooks in release mode..."
cd "${HOOKS_DIR}"

# Build the hooks in release mode
if cargo build --release; then
    BINARY_PATH="${HOOKS_DIR}/target/release/integrated-claude-hooks"
    BINARY_SIZE=$(du -h "${BINARY_PATH}" | cut -f1)

    echo "✅ Build successful!"
    echo "   Binary: ${BINARY_PATH}"
    echo "   Size: ${BINARY_SIZE}"
    echo ""
    echo "🎯 Integrated Claude Code Hooks features:"
    echo "   • Prompt quality evaluation"
    echo "   • Bash command security validation"
    echo "   • Multi-language auto-format/lint (Rust, Node.js, Python)"
    echo "   • MCP tool tracking"
    echo "   • Session statistics"
    echo ""
    echo "📝 Logs will be saved to: ~/.claude/hook-logs/"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "✨ Plugin setup complete!"
echo "   You can now use the plugin in Claude Code"
