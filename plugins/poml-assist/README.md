# poml-assist

Microsoft POML（Prompt Orchestration Markup Language）を活用したプロンプト設計支援プラグイン。

対話フローによる POML プロンプトの生成、構文リファレンスへのアクセス、ファイル保存時の自動バリデーション・レンダリングプレビューを提供する。

## 機能

| 機能 | 説明 |
|---|---|
| **対話フロー** | `/poml-assist:create-poml` コマンドで段階的な質問に答えながら POML を生成 |
| **構文リファレンス** | `.poml` ファイル編集時や POML 関連の質問時に自動でスキルが起動 |
| **自動バリデーション** | `.poml` ファイル保存時に `poml check` を自動実行 |
| **レンダリングプレビュー** | `.poml` ファイル保存時に `poml render` でプレーンテキストプレビューを表示 |
| **POML レビュー** | `poml-reviewer` エージェントが構造的問題・改善提案を 3 段階でレポート |

## インストール要件

バリデーション・レンダリングフックには `poml` CLI が必要:

```bash
pip install poml
```

## 使い方

### コマンド: POML の対話的作成

```
/poml-assist:create-poml
```

または出力ファイルパスを指定:

```
/poml-assist:create-poml prompts/summarize.poml
```

6 フェーズの対話フローで POML プロンプトを作成する:
1. 用途・ペルソナ・入力データの確認
2. 出力形式・Few-shot サンプルの確認
3. テンプレート変数の確認（オプション）
4. POML 生成とプレビュー
5. ファイル保存
6. レビュー案内

### スキル: POML 構文リファレンス

以下のいずれかで自動起動:
- `.poml` ファイルを開いている状態での質問
- 「POML とは」「POML の書き方」「role タグ」などの質問

提供するリファレンス:
- `poml-syntax.md` - ファイル構造・テンプレート変数・条件分岐・ループ
- `poml-tags-reference.md` - 全タグの属性仕様と使用例
- `poml-patterns.md` - 用途別設計パターン集（7 パターン）
- `poml-stylesheet.md` - `<stylesheet>` によるフォーマット制御

### エージェント: POML 自動生成

`poml-architect` エージェントが `/poml-assist:create-poml` コマンドから呼び出され、要件を分析して最適な POML 構造を設計・生成する。

### エージェント: POML レビュー

```
POMLファイルをレビューして
```

`poml-reviewer` エージェントが ERROR / WARNING / INFO の 3 段階でレポートを出力する。

### フック: 自動バリデーション・プレビュー

`.poml` ファイルを保存（Write/Edit）すると自動実行:
1. `validate-poml.py` - `poml check` によるバリデーション
2. `render-poml.py` - `poml render` によるプレーンテキストプレビュー（先頭 300 文字）

フックは advisory のみ（常に exit 0）でブロックしない。

## POML とは

Microsoft が開発した XML 形式のマークアップ言語。AI Agent への構造化した指示記述に使用する。

```xml
<poml>
  <role>あなたは優秀な編集者です。</role>
  <task>以下のテキストを200文字以内で要約してください。</task>
  <output-format>要約文のみを出力。</output-format>
</poml>
```

詳細: https://github.com/microsoft/poml

## ディレクトリ構成

```
plugins/poml-assist/
├── .claude-plugin/plugin.json    # プラグイン設定
├── skills/poml-guide/            # POML 構文リファレンス
│   ├── SKILL.md                  # スキル設定・クイックリファレンス
│   ├── poml-syntax.md            # 構文詳細
│   ├── poml-tags-reference.md    # タグリファレンス
│   ├── poml-patterns.md          # 設計パターン集
│   └── poml-stylesheet.md        # stylesheet ガイド
├── commands/poml-assist/
│   └── create-poml.md            # 対話的 POML 生成コマンド
├── agents/
│   ├── poml-architect.md         # POML 生成エージェント
│   └── poml-reviewer.md          # POML レビューエージェント
├── hooks/
│   ├── hooks.json                # フック設定
│   └── scripts/
│       ├── validate-poml.py      # バリデーションスクリプト
│       └── render-poml.py        # レンダリングプレビュースクリプト
└── README.md
```
