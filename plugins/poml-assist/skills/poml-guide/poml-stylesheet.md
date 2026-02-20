# POML stylesheet ガイド

## 概要

`<stylesheet>` タグを使うことで、モデルの出力スタイルを宣言的に制御できる。
トーン、言語、フォーマット、長さなどを一箇所で統一定義できるため、プロンプトの保守性が向上する。

## 基本構文

```xml
<poml>
  <stylesheet>
    プロパティ: 値
    プロパティ: 値
  </stylesheet>

  <task>タスク内容</task>
</poml>
```

## 主要プロパティ

### `tone` - 文体・トーン

| 値 | 説明 | 用途 |
|---|---|---|
| `professional` | ビジネス文体、フォーマル | 報告書、ビジネスメール |
| `casual` | カジュアル、親しみやすい | チャットボット、FAQ |
| `polite` | 丁寧語、敬語 | カスタマーサポート |
| `technical` | 技術的、専門用語使用可 | 開発者向けドキュメント |
| `academic` | 学術的、論文調 | 研究要約、分析レポート |

```xml
<stylesheet>
  tone: professional
</stylesheet>
```

### `language` - 出力言語

```xml
<stylesheet>
  language: ja        <!-- 日本語 -->
  language: en        <!-- 英語 -->
  language: zh        <!-- 中国語 -->
</stylesheet>
```

### `format` - 出力フォーマット

| 値 | 説明 |
|---|---|
| `markdown` | Markdown 形式（見出し、リスト、コードブロック） |
| `plain` | プレーンテキスト（装飾なし） |
| `json` | JSON 形式（`<output-schema>` との併用推奨） |
| `html` | HTML 形式 |
| `csv` | CSV 形式（表データ出力時） |

```xml
<stylesheet>
  format: markdown
</stylesheet>
```

### `length` - 出力の長さ

| 値 | 説明 |
|---|---|
| `concise` | 簡潔（箇条書きを好む、冗長表現を避ける） |
| `detailed` | 詳細（説明・根拠を含める） |
| `brief` | 極めて短い（1-2 文程度） |

```xml
<stylesheet>
  length: concise
</stylesheet>
```

### `sections` - 含めるセクション

配列形式でレポートのセクション構成を定義できる。

```xml
<stylesheet>
  sections: [summary, analysis, recommendations, conclusion]
</stylesheet>
```

## 組み合わせ例

### ビジネスレポート用

```xml
<poml>
  <stylesheet>
    tone: professional
    language: ja
    format: markdown
    length: detailed
    sections: [executive_summary, findings, recommendations]
  </stylesheet>

  <task>
    四半期の業績データを分析し、役員向けレポートを作成してください。
  </task>
</poml>
```

### カスタマーサポート用

```xml
<poml>
  <stylesheet>
    tone: polite
    language: ja
    format: plain
    length: concise
  </stylesheet>

  <role>
    あなたはカスタマーサポート担当者です。
  </role>

  <task>
    お客様の問い合わせに対して、丁寧かつ簡潔に回答してください。
  </task>
</poml>
```

### 技術ドキュメント用

```xml
<poml>
  <stylesheet>
    tone: technical
    language: en
    format: markdown
    length: detailed
  </stylesheet>

  <role>
    You are a technical writer with expertise in software documentation.
  </role>

  <task>
    Create API documentation for the following endpoint.
  </task>
</poml>
```

### 開発者向けコードレビュー

```xml
<poml>
  <stylesheet>
    tone: technical
    language: ja
    format: markdown
    sections: [issues, suggestions, improved_code]
  </stylesheet>

  <role>
    あなたはコードレビュアーです。建設的なフィードバックを心がけてください。
  </role>

  <task>
    以下のコードをレビューしてください。
  </task>
</poml>
```

## `<output-format>` との使い分け

| | `<stylesheet>` | `<output-format>` |
|---|---|---|
| 用途 | スタイルの宣言的定義 | 出力テンプレートの明示 |
| 詳細度 | 高レベル（プロパティ指定） | 低レベル（具体的なフォーマット） |
| 組み合わせ | 両方使用可能 | 両方使用可能 |

```xml
<poml>
  <!-- 高レベル: Markdown で詳細に -->
  <stylesheet>
    format: markdown
    length: detailed
  </stylesheet>

  <!-- 低レベル: 具体的な構造を指定 -->
  <output-format>
    ## 分析結果

    ### 主要な発見
    [内容]

    ### 推奨アクション
    1. [アクション1]
    2. [アクション2]
  </output-format>

  <task>データを分析してください。</task>
</poml>
```

## 注意事項

- `<stylesheet>` はヒントとして機能する。モデルが完全に遵守することを保証するわけではない
- 厳密なフォーマット制御が必要な場合は `<output-format>` と `<output-schema>` を併用する
- `language` 指定があっても、入力テキストの言語が優先される場合がある（モデル依存）
