#!/bin/bash
# jj safe-push シェルガードのインストーラ
# 現在のシェルの rc ファイルに source 行を追加する
#
# Usage:
#   bash install-safe-push.sh              # 自動検出
#   bash install-safe-push.sh --bash       # .bashrc に追加
#   bash install-safe-push.sh --zsh        # .zshrc に追加
#   bash install-safe-push.sh --uninstall  # 削除

set -euo pipefail

# インストーラ自身の場所からスクリプトのパスを解決
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SAFE_PUSH_SRC="$SCRIPT_DIR/safe-push-shell.sh"
MARKER="# jj safe-push"
SOURCE_LINE="[ -f \"$SAFE_PUSH_SRC\" ] && . \"$SAFE_PUSH_SRC\" $MARKER"

detect_rc_files() {
  local files=()
  case "${1:-}" in
    --bash) files=("$HOME/.bashrc") ;;
    --zsh)  files=("$HOME/.zshrc") ;;
    --uninstall) ;;
    *)
      [ -f "$HOME/.bashrc" ] && files+=("$HOME/.bashrc")
      [ -f "$HOME/.zshrc" ]  && files+=("$HOME/.zshrc")
      ;;
  esac
  printf '%s\n' "${files[@]}"
}

uninstall() {
  local modified=false
  for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
    if [ -f "$rc" ] && grep -qF "$MARKER" "$rc"; then
      grep -vF "$MARKER" "$rc" > "${rc}.tmp" && mv "${rc}.tmp" "$rc"
      printf "Removed from %s\n" "$rc"
      modified=true
    fi
  done
  if ! $modified; then
    printf "jj safe-push の設定は見つかりませんでした。\n"
  fi
  exit 0
}

# メイン処理
if [ "${1:-}" = "--uninstall" ]; then
  uninstall
fi

if [ ! -f "$SAFE_PUSH_SRC" ]; then
  printf "Error: %s が見つかりません。\n" "$SAFE_PUSH_SRC" >&2
  exit 1
fi

rc_files=$(detect_rc_files "${1:-}")
if [ -z "$rc_files" ]; then
  printf "Error: 対象の rc ファイルが見つかりません。--bash または --zsh を指定してください。\n" >&2
  exit 1
fi

installed=false
while IFS= read -r rc; do
  if grep -qF "$MARKER" "$rc" 2>/dev/null; then
    printf "Skip: %s (already installed)\n" "$rc"
  else
    printf '%s\n' "$SOURCE_LINE" >> "$rc"
    printf "Installed: %s\n" "$rc"
    installed=true
  fi
done <<< "$rc_files"

if $installed; then
  printf "\nシェルを再起動するか、以下を実行してください:\n"
  printf "  source %s\n" "$SAFE_PUSH_SRC"
fi
