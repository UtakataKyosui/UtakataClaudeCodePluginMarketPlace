#!/usr/bin/env bash
# jj fix から呼ばれる ruff ラッパー（check --fix + format を1パイプで実行）
# 実行環境（Docker / uvx / ruff）を自動検出する
#
# 使い方: jj-ruff.sh <filename>
#   filename : --stdin-filename に渡すファイル名（jj fix の {name} プレースホルダー）
#   stdin    : ファイル内容
#   stdout   : 修正後のファイル内容
#   exit     : 常に 0（jj fix にファイルを更新させるため）
set -uo pipefail

FNAME="${1:?filename required}"

# ruff check --fix + ruff format をパイプで実行する bash コマンド文字列
# --exit-zero: 修正できない lint エラーが残っていても exit 0 を返す
#   → jj fix がツールの stdout を使ってファイルを更新できるようにする
RUFF_CMD="ruff check --fix --exit-zero --stdin-filename=$(printf '%q' "$FNAME") - | ruff format --stdin-filename=$(printf '%q' "$FNAME") -"

if docker image inspect claude-plugins-lint &>/dev/null 2>&1; then
    # Docker 経由: カレントディレクトリをマウントして pyproject.toml を参照可能にする
    docker run --rm -i \
        -v "$(pwd):/workspace" \
        -w /workspace \
        claude-plugins-lint \
        -c "$RUFF_CMD"
elif command -v uvx &>/dev/null; then
    # uvx 経由（パイプで繋ぐため中間ファイルを使う）
    tmpfile=$(mktemp)
    trap 'rm -f "$tmpfile"' EXIT
    uvx ruff check --fix --exit-zero --stdin-filename="$FNAME" - > "$tmpfile"
    uvx ruff format --stdin-filename="$FNAME" - < "$tmpfile"
elif command -v ruff &>/dev/null; then
    tmpfile=$(mktemp)
    trap 'rm -f "$tmpfile"' EXIT
    ruff check --fix --exit-zero --stdin-filename="$FNAME" - > "$tmpfile"
    ruff format --stdin-filename="$FNAME" - < "$tmpfile"
else
    echo "[jj-ruff] ruff が見つかりません。Docker イメージ(claude-plugins-lint)をビルドするか、" \
         "uv/ruff をインストールしてください。" >&2
    cat  # ファイルを変更せずそのまま出力
fi
