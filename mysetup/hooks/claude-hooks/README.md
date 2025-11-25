# Production Claude Code Hooks

本格的な開発ワークフロー自動化のためのClaude Code フックプログラム

## 🚀 機能

### 💻 Bashコマンド監視・記録
- 実行されるすべてのBashコマンドをリアルタイム記録
- 破壊的コマンド（`rm -rf`等）の検出と警告通知
- システムレベルコマンド（`sudo`等）の分類
- コマンド実行履歴のタイムスタンプ付きログ

### 🦀 Rust自動化
- Rustファイル操作時の自動`cargo fmt`実行
- 自動`cargo clippy`によるリント
- プロジェクトルート自動検出
- フォーマット・リント実行結果の記録

### 🔌 MCP統合監視
- すべてのMCPツール使用の追跡
- サーバー別使用統計（Context7, Magic, Playwright等）
- ツールカテゴリ分類（ドキュメント、UI生成、ブラウザ自動化等）
- リソース集約的操作の検出と通知

### 📚 情報収集記録
- ドキュメント検索クエリと結果の記録
- 情報収集セッションの追跡
- 検索結果の概要保存

### 📊 セッション統計
- セッション時間の計測
- ツール使用頻度の分析
- 自動化適用率の追跡
- JSONフォーマットでの詳細ログ出力

## 📥 インストール

### 1. バイナリのビルドとインストール

```bash
# プロジェクトディレクトリに移動
cd ~/claude-hooks

# リリースビルド
cargo build --release

# バイナリを適切な場所にコピー
sudo cp target/release/production-hooks /usr/local/bin/claude-production-hooks

# 実行権限を付与
sudo chmod +x /usr/local/bin/claude-production-hooks
```

### 2. Claude Code設定への追加

Claude Codeの設定ファイル（`.claude/settings.json`）にフックを追加：

```json
{
  "hooks": {
    "pre_tool_use": [
      {
        "command": "/usr/local/bin/claude-production-hooks",
        "timeout": 30
      }
    ],
    "post_tool_use": [
      {
        "command": "/usr/local/bin/claude-production-hooks",
        "timeout": 30
      }
    ],
    "stop": [
      {
        "command": "/usr/local/bin/claude-production-hooks",
        "timeout": 30
      }
    ]
  }
}
```

## 🎯 使用方法

### 自動実行
Claude Codeが実行されると自動的にフックが作動し、以下の機能が有効になります：

- **Bashコマンド実行時**: コマンドの分類と記録
- **ファイル操作時**: Rustファイルの自動フォーマット・リント
- **MCPツール使用時**: 使用統計の記録と分類
- **セッション終了時**: 統計の表示と保存

### ログファイル
セッションデータは以下の場所に保存されます：
```
~/.claude/hook-logs/session-{session_id}.json
```

### 手動テスト
```bash
# Bashコマンドのテスト
echo '{"hook_event_name":"PreToolUse","session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","tool_name":"Bash","tool_input":{"command":"ls -la"}}' | claude-production-hooks

# Rustファイル操作のテスト
echo '{"hook_event_name":"PreToolUse","session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","tool_name":"Write","tool_input":{"file_path":"test.rs","content":"fn main() {}"}}' | claude-production-hooks

# MCPツールのテスト
echo '{"hook_event_name":"PreToolUse","session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","tool_name":"mcp__context7__get-library-docs","tool_input":{"libraryName":"tokio"}}' | claude-production-hooks
```

## 📊 出力例

### セッション実行中
```
🎯 Production Claude Code Hooks activated
   Features: Bash logging, Rust automation, MCP tracking, Info recording
   Logs saved to: ~/.claude/hook-logs/

🚀 Starting session: session-12345
💻 [BASH] cargo build
🦀 [RUST] Write -> src/main.rs
  🔧 Running Rust automation for: src/main.rs
    ✨ Formatted with cargo fmt
    🔍 Checked with cargo clippy
🔌 [MCP] Using get-library-docs (context7) - Documentation and research
  📚 Gathering information...
  📚 Information gathering completed
```

### セッション終了時統計
```
🛑 Session ended: session-12345

📊 Session Summary (session-12345)
  ⏱️  Duration: 15 minutes
  💻 Bash Commands: 8
    🔧 System-level commands: 2
  🦀 Rust Operations: 3
    ✨ Auto-formatted: 3
    🔍 Auto-linted: 3  
  🔌 MCP Usage:
    context7 x4
    magic x2
  📚 Information Sessions: 4
```

## ⚙️ カスタマイズ

### 破壊的コマンドパターンの追加
`src/main.rs`の`is_destructive_command`関数でパターンを編集：

```rust
let destructive_patterns = [
    "rm -rf", "rm -r", "sudo rm", "rm /", "del ", "rmdir",
    "format ", "fdisk", "dd if=", "sudo dd", "> /dev/",
    "sudo chmod 777", "chmod -R 777", "sudo chown -R",
    // カスタムパターンを追加
    "your-custom-pattern"
];
```

### システムレベルコマンドパターンの追加
`is_system_level_command`関数で追加：

```rust
let system_patterns = [
    "sudo ", "su ", "doas ", "systemctl", "/etc/", "/var/",
    "your-system-pattern"
];
```

## 🔧 トラブルシューティング

### フックが実行されない
1. バイナリパスが正しいことを確認
2. 実行権限があることを確認
3. Claude Codeの設定ファイルの構文をチェック

### Rust自動化が動作しない  
1. `cargo`と`rustfmt`がインストール済みか確認
2. プロジェクトルートに`Cargo.toml`があるか確認
3. ファイルパスが存在するか確認

### 通知が表示されない
- macOS: システム環境設定でターミナルの通知を許可
- Linux: `notify-send`がインストール済みか確認
- Windows: PowerShellの実行ポリシーを確認

## 📚 ログファイル形式

セッションログは以下のJSON形式で保存されます：

```json
{
  "session_id": "session-12345",
  "start_time": "2024-01-01T12:00:00Z",
  "bash_commands": [
    {
      "timestamp": "2024-01-01T12:00:30Z",
      "command": "cargo build",
      "session_id": "session-12345",
      "cwd": "/project",
      "is_destructive": false,
      "is_system_level": false
    }
  ],
  "rust_operations": [
    {
      "timestamp": "2024-01-01T12:01:00Z",
      "operation_type": "Write",
      "file_path": "src/main.rs",
      "session_id": "session-12345",
      "lint_applied": true,
      "format_applied": true
    }
  ],
  "mcp_usage": {
    "context7": 4,
    "magic": 2
  },
  "info_gathering_sessions": [
    {
      "timestamp": "2024-01-01T12:02:00Z",
      "session_id": "session-12345",
      "tool_name": "get-library-docs",
      "query_info": {"libraryName": "tokio"},
      "result_summary": "Retrieved: Documentation for tokio async runtime..."
    }
  ]
}
```

## 🤝 開発への貢献

このプロジェクトは[claude-code-hooks-lib](https://github.com/UtakataKyosui/claude-code-hooks-lib)ライブラリを使用しています。

---

**注意**: このフックプログラムは開発ワークフローの効率化を目的としています。本番環境での使用前に十分なテストを行ってください。