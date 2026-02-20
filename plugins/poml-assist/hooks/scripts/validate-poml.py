#!/usr/bin/env python3
"""
PostToolUse hook: POML バリデーション（Advisory のみ）

Write/Edit ツール使用後に実行され、.poml ファイルに対して
`poml check` を実行してバリデーション結果を表示する。

- .poml 拡張子以外はスキップ
- poml CLI 未インストール時はインストール案内を表示して exit 0
- 常に exit 0（advisory のみ、ブロックしない）
"""

import json
import os
import subprocess
import sys

MAX_FILE_SIZE = 512 * 1024  # 512KB


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
    # 1. 入力パスを正規化し、明示的な親ディレクトリ参照を拒否する
    normalized = os.path.normpath(file_path)
    if ".." in normalized.split(os.sep):
        return False, "パストラバーサルが検出されました"

    # 2. 実際のパスを解決し、プロジェクトルート配下か確認する
    real_path = os.path.realpath(file_path)
    project_root = os.path.realpath(os.getcwd())
    try:
        common_prefix = os.path.commonpath([project_root, real_path])
    except ValueError:
        return False, "パストラバーサルが検出されました"
    if common_prefix != project_root:
        return False, "プロジェクト外のパスが検出されました"

    # ファイル存在チェック
    if not os.path.isfile(real_path):
        return False, f"ファイルが存在しません: {file_path}"

    # ファイルサイズチェック
    file_size = os.path.getsize(real_path)
    if file_size > MAX_FILE_SIZE:
        return False, f"ファイルサイズが上限（512KB）を超えています: {file_size} bytes"

    return True, None


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


def run_validation(file_path):
    """poml check を実行してバリデーションを行う"""
    try:
        result = subprocess.run(
            ["poml", "check", file_path],
            capture_output=True,
            text=True,
            timeout=30,
        )
        return result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return -1, "", "タイムアウト（30秒）が発生しました"
    except (FileNotFoundError, OSError) as e:
        return -1, "", str(e)


def main():
    data = read_input()
    file_path = extract_file_path(data)

    if not file_path:
        sys.exit(0)

    # POML ファイル以外はスキップ
    if not is_poml_file(file_path):
        sys.exit(0)

    # ファイル安全性チェック
    is_safe, error_msg = check_file_safety(file_path)
    if not is_safe:
        print(f"[POML validate] スキップ: {error_msg}", file=sys.stderr)
        sys.exit(0)

    # poml CLI の存在確認
    if not check_poml_cli():
        print(
            "[POML validate] poml CLI がインストールされていません。\n"
            "  インストール方法: pip install poml\n"
            "  インストール後、.poml ファイルの保存時に自動バリデーションが実行されます。",
            file=sys.stderr,
        )
        sys.exit(0)

    # バリデーション実行
    returncode, stdout, stderr = run_validation(file_path)

    if returncode == 0:
        print(
            f"[POML validate] ✓ {os.path.basename(file_path)} のバリデーションが成功しました。"
        )
    else:
        error_output = stderr.strip() or stdout.strip()
        print(
            f"[POML validate] ⚠ {os.path.basename(file_path)} にエラーがあります:\n"
            f"{error_output}\n\n"
            "  修正するには `/poml-assist:create-poml` コマンドを使用してください。",
            file=sys.stderr,
        )

    sys.exit(0)


if __name__ == "__main__":
    main()
