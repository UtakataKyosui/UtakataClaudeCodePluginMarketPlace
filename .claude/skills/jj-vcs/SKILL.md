---
name: jj-vcs
description: Jujutsu (jj) VCSを使用する際のコマンドガイド。GitからJJへの移行、jjコマンドの使い方、バージョン管理操作を行う際に参照する。
---

# Jujutsu (jj) VCS コマンドガイド

Jujutsu（jj）は、Gitと互換性を持ちながらよりシンプルで強力なワークフローを提供する次世代バージョン管理システムです。

## 基本概念

- **チェンジ（Change）**: jjでは「コミット」ではなく「チェンジ」という概念を使用
- **ワーキングコピー**: 常に自動的にスナップショットされる
- **ブックマーク**: Gitのブランチに相当する概念
- **操作ログ**: すべての操作履歴を追跡し、`jj undo`で取り消し可能

## GitコマンドとJJコマンドの対応表

### リポジトリ操作

| Git | jj | 説明 |
|-----|-----|------|
| `git init` | `jj git init [--no-colocate]` | リポジトリ初期化 |
| `git clone <url>` | `jj git clone <url>` | リモートリポジトリをクローン |
| `git fetch` | `jj git fetch [--remote <remote>]` | リモートから変更を取得 |
| `git push --all` | `jj git push --all [--remote <remote>]` | すべての変更をプッシュ |
| `git push -u origin <branch>` | `jj git push -b <bookmark>` | 特定のブックマークをプッシュ |

### 変更操作

| Git | jj | 説明 |
|-----|-----|------|
| `git status` | `jj status` または `jj st` | 現在の状態を表示 |
| `git diff` | `jj diff` | 差分を表示 |
| `git diff HEAD` | `jj diff` | 現在のチェンジの差分 |
| `git add -A && git commit` | `jj commit` または `jj ci` | 変更をコミット |
| `git commit --amend` | `jj squash` | 変更を親チェンジに統合 |
| `git reset --hard` | `jj abandon` | チェンジを放棄 |
| `git checkout <commit>` | `jj edit <revision>` | 特定のリビジョンを編集 |
| `git stash` | （不要） | jjでは自動的にスナップショット |

### コミットメッセージ操作

| Git | jj | 説明 |
|-----|-----|------|
| `git commit --amend` | `jj describe` または `jj desc` | チェンジの説明を編集 |
| `git commit -m "msg"` | `jj commit -m "msg"` | メッセージを指定してコミット |
| （なし） | `jj describe -m "msg"` | 現在のチェンジの説明を更新 |

### ブランチ/ブックマーク操作

| Git | jj | 説明 |
|-----|-----|------|
| `git branch` | `jj bookmark list` | ブックマーク一覧 |
| `git branch <name>` | `jj bookmark create <name> -r <revision>` | ブックマーク作成 |
| `git branch -d <name>` | `jj bookmark delete <name>` | ブックマーク削除 |
| `git branch -f <name>` | `jj bookmark move <name> --to <revision>` | ブックマーク移動 |
| `git checkout -b <name>` | `jj new` + `jj bookmark create <name>` | 新規ブックマーク作成 |

### 履歴操作

| Git | jj | 説明 |
|-----|-----|------|
| `git log` | `jj log` | 履歴表示 |
| `git log --oneline` | `jj log --no-graph` | シンプルな履歴表示 |
| `git rebase <base> <target>` | `jj rebase -b <target> -d <base>` | リベース |
| `git cherry-pick <commit>` | `jj new <revision>` | 特定のリビジョンから新規チェンジ |
| `git revert <commit>` | `jj backout -r <revision>` | リビジョンを打ち消し |

### 高度な操作

| Git | jj | 説明 |
|-----|-----|------|
| `git commit -p` | `jj split` | チェンジを分割 |
| `git reflog` | `jj op log` | 操作ログを表示 |
| （なし） | `jj undo` | 直前の操作を取り消し |
| `git merge` | `jj new <rev1> <rev2>` | マージ（複数の親を持つチェンジ） |
| `git diff <a> <b>` | `jj diff --from <a> --to <b>` | 2つのリビジョン間の差分 |

## 主要コマンド詳細

### `jj new` - 新規チェンジ作成

```bash
# 現在のチェンジの子として新規チェンジを作成
jj new

# 特定のリビジョンから新規チェンジを作成
jj new <revision>

# 複数の親を持つチェンジを作成（マージ）
jj new <rev1> <rev2>
```

### `jj describe` - チェンジ説明の編集

```bash
# 現在のチェンジの説明を編集
jj describe

# メッセージを直接指定
jj describe -m "変更内容の説明"

# 特定のリビジョンの説明を編集
jj describe -r <revision>
```

### `jj commit` - コミット

```bash
# 現在のワーキングコピーをコミット
jj commit

# メッセージを指定してコミット
jj commit -m "コミットメッセージ"
```

### `jj squash` - 変更の統合

```bash
# 現在のチェンジを親に統合
jj squash

# 特定のチェンジを親に統合
jj squash -r <revision>
```

### `jj rebase` - リベース

```bash
# 現在のチェンジを別の親に移動
jj rebase -d <destination>

# 特定のリビジョンをリベース
jj rebase -r <revision> -d <destination>

# ブランチ全体をリベース
jj rebase -b <branch> -d <destination>
```

### `jj bookmark` - ブックマーク管理

```bash
# ブックマーク一覧
jj bookmark list

# ブックマーク作成
jj bookmark create <name> -r <revision>

# ブックマーク移動
jj bookmark move <name> --to <revision>

# ブックマーク削除
jj bookmark delete <name>
```

### `jj git` - Git連携

```bash
# Gitリモートから取得
jj git fetch

# Gitリモートにプッシュ
jj git push --all
jj git push -b <bookmark>

# 特定のリモートを指定
jj git fetch --remote origin
jj git push --remote origin
```

## リビジョン指定方法

jjでは柔軟なリビジョン指定が可能です：

| 指定方法 | 説明 |
|---------|------|
| `@` | 現在のワーキングコピー |
| `@-` | 現在のワーキングコピーの親 |
| `@--` | 現在のワーキングコピーの親の親 |
| `<change_id>` | チェンジID（短縮形可） |
| `<commit_id>` | コミットID（短縮形可） |
| `<bookmark>` | ブックマーク名 |
| `root()` | ルートコミット |
| `heads()` | すべてのヘッド |

## ベストプラクティス

1. **頻繁に`jj status`を確認**: 現在の状態を把握する
2. **`jj undo`を活用**: 間違えても簡単に戻せる
3. **`jj describe`でこまめに説明を更新**: 作業中でも説明を残す
4. **ステージングは不要**: jjではすべての変更が自動的にトラッキングされる
5. **`jj op log`で操作履歴を確認**: 何をしたか忘れた時に便利

## ヘルプの確認

より詳細な情報は以下で確認できます：

```bash
# 全体のヘルプ
jj help

# 特定コマンドのヘルプ
jj help <command>
```

## 参考リンク

- 公式ドキュメント: https://www.jj-vcs.dev/
- GitコマンドとJJコマンドの対応表: https://www.jj-vcs.dev/latest/git-command-table/
- CLIリファレンス: https://www.jj-vcs.dev/latest/cli-reference/
