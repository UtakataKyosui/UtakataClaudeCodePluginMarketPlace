#!/usr/bin/env bash
# jj fix から呼ばれる ruff ラッパー（check --fix + format を1パイプで実行）
# 実行環境（Docker / uvx / ruff）を自動検出する
#
# 使い方: jj-ruff.sh <filename>
#   filename : --stdin-filename に渡すファイル名（jj fix の $path プレースホルダー）
#   stdin    : ファイル内容
#   stdout   : 修正後のファイル内容
#   exit     : 常に 0（jj fix にファイルを更新させるため）
set -uo pipefail

FNAME="${1:?filename required}"

# ruff check --fix + ruff format をパイプで実行する bash コマンド文字列
# --exit-zero: 修正できない lint エラーが残っていても exit 0 を返す
#   → jj fix がツールの stdout を使ってファイルを更新できるようにする
# シングルクォートで固定部を囲み、printf '%q' でファイル名をシェルエスケープする（コマンドインジェクション対策）
RUFF_CMD='ruff check --fix --exit-zero --stdin-filename='$(printf '%q' "$FNAME")' - | ruff format --stdin-filename='$(printf '%q' "$FNAME")' -'

if docker image inspect claude-plugins-lint &>/dev/null; then
    # Docker 経由: カレントディレクトリをマウントして pyproject.toml を参照可能にする
    docker run --rm -i \
        -v "$(pwd):/workspace" \
        -w /workspace \
        claude-plugins-lint \
        -c "$RUFF_CMD"
elif command -v uvx &>/dev/null; then
    # uvx 経由（パイプで stdin → check → format → stdout と流す）
    uvx ruff check --fix --exit-zero --stdin-filename="$FNAME" - | uvx ruff format --stdin-filename="$FNAME" -
elif command -v ruff &>/dev/null; then
    ruff check --fix --exit-zero --stdin-filename="$FNAME" - | ruff format --stdin-filename="$FNAME" -
else
    echo "[jj-ruff] ruff が見つかりません。Docker イメージ(claude-plugins-lint)をビルドするか、" \
         "uv/ruff をインストールしてください。" >&2
    cat  # ファイルを変更せずそのまま出力
fi

exit 0
