# Claude Code Hooks - macOS 通知・権限確認・自動フォーマット対応

このプロジェクトは、Claude Codeの実行時に以下の機能を提供するフックシステムです：

## 主な機能

### 1. macOS通知システム
- **プロンプト送信時**: プロンプト品質スコアを通知
- **ツール使用前**: ツール使用開始の通知
- **危険なコマンド**: 潜在的に危険な操作の警告通知
- **ファイル編集完了**: 編集作業完了後の通知
- **フォーマット/リント完了**: 自動フォーマット実行結果の通知

### 2. プロンプト品質チェック
- **スコア評価**: 100点満点でプロンプトの品質を評価
- **自動ブロック**: 60点未満のプロンプトを自動的にブロック
- **品質基準**:
  - 文字数制限 (15-500文字)
  - 曖昧な表現の回避
  - 具体的なタスク動詞の使用
  - プログラミング関連の内容
  - 安全性・倫理面での問題の検出

### 3. 権限確認通知
- **危険コマンド検出**: `rm -rf`, `sudo rm`, `chmod 777` 等の危険な操作を検出
- **事前通知**: 潜在的に危険な操作の実行前に警告
- **ログ記録**: すべての操作を詳細にログファイルに記録

### 4. 自動フォーマット・リント
**Rustプロジェクト**:
- `cargo fmt` (自動フォーマット)
- `cargo clippy` (静的解析・リント)

**Node.jsプロジェクト**:
- `prettier --write .` または `npm run format`
- `eslint . --fix` または `npm run lint`

**Pythonプロジェクト**:
- `black .` (自動フォーマット)
- `flake8 .` (リント)

### 5. ログ記録システム
- **ログ保存場所**: `~/.claude/hook-logs/hook.log`
- **記録内容**: タイムスタンプ付きでイベント詳細を記録
- **イベント種類**: プロンプト送信、ツール使用、危険コマンド、フォーマット結果等

## インストール

### 🚀 自動インストール (推奨)

**macOS専用**: ワンライナーでインストール

```bash
curl -fsSL https://raw.githubusercontent.com/UtakataKyosui/MyClaudeCodeHooks/main/install.sh | bash
```

### 📦 手動インストール

#### 方法1: GitHub Actions からダウンロード

1. [GitHub Actions](https://github.com/UtakataKyosui/MyClaudeCodeHooks/actions) にアクセス
2. 最新の成功したワークフローを選択  
3. `myclaudecodehooks-macos` アーティファクトをダウンロード
4. ダウンロードしたファイルを展開
5. インストール:
   ```bash
   chmod +x MyClaudeCodeHooks
   mkdir -p ~/.local/bin
   mv MyClaudeCodeHooks ~/.local/bin/
   ```

#### 方法2: ソースからビルド

```bash
git clone https://github.com/UtakataKyosui/MyClaudeCodeHooks.git
cd MyClaudeCodeHooks
cargo build --release
cp target/release/MyClaudeCodeHooks ~/.local/bin/
```

### ⚙️ セットアップ

#### 1. PATHの設定
`~/.zshrc` または `~/.bash_profile` に追加:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

#### 2. Claude Code での設定
```bash
claude-code config set hooks.command "$HOME/.local/bin/MyClaudeCodeHooks"
```

#### 3. 必要な権限
- **macOS通知権限**: システム環境設定 > 通知 で Claude Code Hook の通知を許可
- **ファイルアクセス権限**: ログファイル作成のためのホームディレクトリアクセス

### ✅ インストール確認
```bash
MyClaudeCodeHooks --version  # バージョン情報表示
```

## 使用例

### プロンプト品質チェック
```
低品質プロンプト例: "なんでもいいので適当にコードを書いて"
→ スコア: 25/100 → ブロック & 警告通知

高品質プロンプト例: "React でユーザー認証フォームを実装してください。50行以内でお願いします。"
→ スコア: 85/100 → 受理 & 確認通知
```

### 危険コマンド検出
```
危険な操作: bash の中で "rm -rf /" のようなコマンド
→ 即座に警告通知 & ログ記録
```

### 自動フォーマット
```
Edit/MultiEdit/Write ツール使用後
→ 自動的に該当プロジェクトのフォーマット・リントを実行
→ 結果を macOS 通知で報告
```

## ログファイル形式
```
[2024-01-20 10:30:45 UTC] PROMPT_SUBMIT: スコア: 85, プロンプト: React でユーザー認証フォームを実装してください...
[2024-01-20 10:31:20 UTC] PRE_TOOL_USE: ツール: Write
[2024-01-20 10:31:25 UTC] POST_TOOL_USE: ツール 'Write' の実行が完了しました
[2024-01-20 10:31:30 UTC] FORMAT_LINT: ✅ prettier フォーマット完了
✅ eslint 完了
```

## カスタマイズ

### プロンプト品質しきい値の変更
`src/main.rs` の `THRESHOLD` 定数を変更：
```rust
const THRESHOLD: i32 = 60; // デフォルト値
```

### 危険コマンドパターンの追加
`is_dangerous_command` 関数の `dangerous_patterns` 配列に追加：
```rust
let dangerous_patterns = [
    r"rm\s+-rf",
    // 新しいパターンを追加
];
```

### フォーマット・リントコマンドの変更
`run_format_and_lint` 関数内の各言語セクションでコマンドをカスタマイズ可能

## トラブルシューティング

### 通知が表示されない
1. システム環境設定 > 通知 で Claude Code Hook を確認
2. 通知の許可設定を有効にする

### フォーマット・リントが動作しない
1. 該当ツール (prettier, eslint, cargo, black 等) がインストールされているか確認
2. プロジェクトルートで手動実行してエラーメッセージを確認

### ログが記録されない
1. `~/.claude/hook-logs/` ディレクトリの書き込み権限を確認
2. ディスク容量を確認

## 開発者向け情報

### 依存関係
- `claude-code-hooks`: Claude Code フック API
- `mac-notification-sys`: macOS通知システム
- `serde`: JSON シリアライゼーション
- `chrono`: 日時処理
- `regex`: 正規表現
- `dirs`: ディレクトリ操作

### アーキテクチャ
- **モジュール設計**: 各機能が独立したモジュールとして実装
- **非同期処理**: ツール操作との並行実行をサポート
- **エラーハンドリング**: 堅牢なエラー処理とログ記録
- **設定可能**: 閾値やパターンは容易にカスタマイズ可能