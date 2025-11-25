#!/bin/bash

# MyClaudeCodeHooks インストールスクリプト
# macOS専用

set -e

# 色設定
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# バージョン情報
REPO="UtakataKyosui/MyClaudeCodeHooks"
BINARY_NAME="MyClaudeCodeHooks"
INSTALL_DIR="$HOME/.local/bin"

echo -e "${BLUE}MyClaudeCodeHooks インストーラー${NC}"
echo "======================================"

# macOSチェック
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}エラー: このツールはmacOSでのみ動作します${NC}"
    exit 1
fi

# インストールディレクトリ作成
echo -e "${YELLOW}インストールディレクトリを作成中...${NC}"
mkdir -p "$INSTALL_DIR"

# 最新リリースのダウンロードURL取得
echo -e "${YELLOW}最新リリース情報を取得中...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")

if echo "$LATEST_RELEASE" | grep -q "Not Found"; then
    echo -e "${YELLOW}リリースが見つかりません。最新のビルド成果物をダウンロードします...${NC}"
    
    # GitHub Actions の最新成果物をダウンロード
    echo -e "${YELLOW}最新のビルド成果物をダウンロードしています...${NC}"
    echo -e "${BLUE}手動でダウンロードしてください:${NC}"
    echo "1. https://github.com/$REPO/actions にアクセス"
    echo "2. 最新の成功したワークフローを選択"
    echo "3. 'myclaudecodehooks-macos' アーティファクトをダウンロード"
    echo "4. ダウンロードしたファイルを展開"
    echo "5. 以下のコマンドでインストール:"
    echo "   chmod +x MyClaudeCodeHooks"
    echo "   mv MyClaudeCodeHooks $INSTALL_DIR/"
    echo ""
    echo -e "${YELLOW}または、手動インストール用の指示に従ってください${NC}"
    exit 0
else
    # リリースからダウンロード
    DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o '"browser_download_url": "[^"]*' | cut -d'"' -f4 | head -1)
    
    if [ -z "$DOWNLOAD_URL" ]; then
        echo -e "${RED}エラー: ダウンロードURLが見つかりません${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}ダウンロード中: $DOWNLOAD_URL${NC}"
    curl -L -o "/tmp/$BINARY_NAME" "$DOWNLOAD_URL"
fi

# バイナリをインストール
if [ -f "/tmp/$BINARY_NAME" ]; then
    echo -e "${YELLOW}バイナリをインストール中...${NC}"
    chmod +x "/tmp/$BINARY_NAME"
    mv "/tmp/$BINARY_NAME" "$INSTALL_DIR/"
    
    echo -e "${GREEN}✅ インストール完了!${NC}"
    echo ""
    echo -e "${BLUE}インストール場所:${NC} $INSTALL_DIR/$BINARY_NAME"
    echo ""
    echo -e "${YELLOW}PATHに追加するには、以下を ~/.zshrc または ~/.bash_profile に追加してください:${NC}"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo -e "${YELLOW}Claude Code でフックを有効にするには:${NC}"
    echo "claude-code config set hooks.command \"$INSTALL_DIR/$BINARY_NAME\""
    echo ""
    echo -e "${GREEN}インストールが完了しました! 🎉${NC}"
fi