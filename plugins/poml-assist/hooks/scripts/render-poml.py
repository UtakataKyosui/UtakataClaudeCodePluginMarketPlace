#!/usr/bin/env python3
"""
PostToolUse hook: POML レンダリングプレビュー（Advisory のみ）

Write/Edit ツール使用後に実行され、.poml ファイルに対して
`poml render` を実行してプレーンテキストのプレビューを表示する。

- .poml 拡張子以外はスキップ
- poml CLI 未インストール時はスキップ（validate-poml.py で案内済み）
- 常に exit 0（advisory のみ、ブロックしない）
"""

import json
import os
import subprocess
import sys

MAX_FILE_SIZE = 512 * 1024  # 512KB
PREVIEW_MAX_CHARS = 300


def read_input():
    """stdin から PostToolUse の JSON データを読み取る"""
    try:
        return json.loads(sys.stdin.read())
    except (json.JSONDecodeError, EOFError, OSError, UnicodeDecodeError):
        return None


def extract_file_path(data):
    """ツール実行データからファイルパスを抽出する"""
    if not data:
        return None
    tool_input = data.get("tool_input", {})
    return tool_input.get("file_path")


def is_poml_file(file_path):
    """POML ファイルかどうか判定する"""
    return file_path.endswith(".poml")


def check_file_safety(file_path):
    """ファイルのサイズとパスを検証する"""
    # パストラバーサル対策
    if ".." in file_path:
        return False

    real_path = os.path.realpath(file_path)

    # ファイル存在チェック
    if not os.path.isfile(real_path):
        return False

    # ファイルサイズチェック
    file_size = os.path.getsize(real_path)
    return file_size <= MAX_FILE_SIZE


def check_poml_cli():
    """poml CLI がインストールされているか確認する"""
    try:
        result = subprocess.run(
            ["poml", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
        )
        return result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError, OSError):
        return False


def run_render(file_path):
    """poml render を実行してプレーンテキストに変換する"""
    try:
        result = subprocess.run(
            ["poml", "render", file_path],
            capture_output=True,
            text=True,
            timeout=30,
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "タイムアウト（30秒）が発生しました"
    except (FileNotFoundError, OSError) as e:
        return -1, "", str(e)


def format_preview(rendered_text, file_name):
    """レンダリング結果をプレビュー形式にフォーマットする"""
    if not rendered_text.strip():
        return f"[POML render] {file_name} のレンダリング結果が空です。"

    if len(rendered_text) <= PREVIEW_MAX_CHARS:
        return f"[POML render] {file_name} のプレビュー:\n{rendered_text}"

    preview = rendered_text[:PREVIEW_MAX_CHARS]
    return (
        f"[POML render] {file_name} のプレビュー（先頭 {PREVIEW_MAX_CHARS} 文字）:\n"
        f"{preview}\n...\n"
        f"  全文確認: poml render {file_name}"
    )


def main():
    data = read_input()
    file_path = extract_file_path(data)

    if not file_path:
        sys.exit(0)

    # POML ファイル以外はスキップ
    if not is_poml_file(file_path):
        sys.exit(0)

    # ファイル安全性チェック
    if not check_file_safety(file_path):
        sys.exit(0)

    # poml CLI の存在確認（未インストール時はスキップのみ、案内は validate-poml.py が担当）
    if not check_poml_cli():
        sys.exit(0)

    # レンダリング実行
    returncode, stdout, stderr = run_render(file_path)

    if returncode == 0:
        preview_msg = format_preview(stdout, os.path.basename(file_path))
        print(preview_msg)
    else:
        error_output = stderr.strip() or stdout.strip()
        print(
            f"[POML render] ⚠ {os.path.basename(file_path)} のレンダリングに失敗しました:\n"
            f"{error_output}",
            file=sys.stderr,
        )

    sys.exit(0)


if __name__ == "__main__":
    main()
