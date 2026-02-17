# Safe Push Workflow

`jj git push` の直接実行を禁止し、fetch → conflict 確認 → dry-run → push の安全なワークフローを強制する仕組み。

## 背景

jj はコミットの書き換えを前提とした VCS のため、`jj git push` 時にリモートと異なるコミットがあると自動的に force-push する。これにより、他の人がリモートに push した変更を上書きしてしまうリスクがある。

## ワークフロー

### Step 1: リモートの最新状態を取得

```bash
jj git fetch
```

### Step 2: bookmark の diverge を確認

```bash
jj bookmark list --conflicted
```

- 出力が空であれば diverge なし。Step 3 に進む。
- conflicted な bookmark がある場合は解消してから再実行する。

### Step 3: dry-run で push 内容を確認

```bash
jj git push --dry-run
```

push される内容を確認する。force-push が含まれる場合はその旨を明示する。

### Step 4: push を実行

ユーザーの確認を得てから push を実行する。

```bash
jj git push
```

特定の bookmark のみを push する場合:

```bash
jj git push -b <bookmark-name>
```

## ガード機構（二重防御）

### Claude Code hook（エージェント操作）

`hooks/scripts/jj-safe-push.sh` が PreToolUse hook として提供されており、以下の動作を強制する:

- `jj git push` の直接実行は常にブロックされる
- `jj git push --dry-run` のみ許可される
- dry-run 実行後に限り、1回の `jj git push` が許可される（次回は再度 dry-run が必要）

### シェル関数ラッパー（ユーザー操作）

`hooks/scripts/safe-push-shell.sh` がシェル関数として `jj` コマンドをラップし、ターミナルでの直接操作もガードする:

- `jj git push` → ブロックし `jj safe-push` の使用を案内
- `jj git push --dry-run` → 許可
- `jj safe-push` → fetch → conflict 確認 → dry-run → 確認 → push の安全なワークフローを対話的に実行

### セットアップ

```bash
# Claude Code hook のインストール（hooks.json で自動適用）

# シェルガードのインストール
bash plugins/jj-vcs-workflow/hooks/scripts/install-safe-push.sh          # 自動検出
bash plugins/jj-vcs-workflow/hooks/scripts/install-safe-push.sh --bash    # .bashrc のみ
bash plugins/jj-vcs-workflow/hooks/scripts/install-safe-push.sh --zsh     # .zshrc のみ
bash plugins/jj-vcs-workflow/hooks/scripts/install-safe-push.sh --uninstall  # 削除
```

## diverge の解消

conflicted bookmark が検出された場合の対処:

1. **rebase で解消**: `jj rebase -b <bookmark> -d <target>` でローカルの変更をリモートの上に rebase
2. **手動で確認**: `jj log` で状況を確認し、判断する
3. **push を中止**: 安全のため push を行わない
