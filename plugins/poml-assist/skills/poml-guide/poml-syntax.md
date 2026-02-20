# POML 構文リファレンス

## ファイル構造

POML ファイルは XML 形式で、`.poml` 拡張子を使用する。

```xml
<?xml version="1.0" encoding="UTF-8"?>
<poml>
  <!-- ここに要素を配置 -->
</poml>
```

XML 宣言は省略可能。`<poml>` がルート要素として必須。

## 必須要素

### `<task>`

実行するタスクを定義する。**必須タグ**（省略不可）。

```xml
<poml>
  <task>
    以下のテキストを要約してください。
    - 要点を 3 点以内にまとめる
    - 専門用語は避ける
  </task>
</poml>
```

## テンプレート変数

`{{variable_name}}` 構文で変数を埋め込む。

```xml
<poml>
  <task>
    以下の{{document_type}}を{{target_language}}に翻訳してください。
  </task>
</poml>
```

### `<let>` による変数定義

デフォルト値を持つ変数を定義できる。

```xml
<poml>
  <let name="language">日本語</let>
  <let name="max_length">500</let>

  <task>
    テキストを{{language}}に翻訳し、{{max_length}}文字以内にまとめてください。
  </task>
</poml>
```

### CLI からの変数渡し

```bash
poml render prompt.poml --var language=英語 --var max_length=300
```

## 条件分岐

POML は XML ベースのため、条件分岐は変数と `<let>` の組み合わせで表現する。

```xml
<poml>
  <let name="mode">detailed</let>

  <task>
    {{mode}} モードで分析してください。
    <!-- detailed: 詳細な分析 / brief: 簡潔な要約 -->
  </task>
</poml>
```

## コメント

標準 XML コメント構文を使用する。

```xml
<poml>
  <!-- このプロンプトは v2.0 で追加 -->
  <task>
    <!-- TODO: Few-shot サンプルを追加する -->
    タスクの説明...
  </task>
</poml>
```

## 文字エスケープ

XML の特殊文字はエスケープが必要。

| 文字 | エスケープ |
|---|---|
| `<` | `&lt;` |
| `>` | `&gt;` |
| `&` | `&amp;` |
| `"` | `&quot;` |
| `'` | `&apos;` |

```xml
<poml>
  <task>
    JSON の &lt;key&gt; フィールドを分析してください。
  </task>
</poml>
```

## CDATA セクション

JSON やコードブロックには CDATA を使うと便利。

```xml
<poml>
  <task>
    以下の JSON を解析してください:
    <![CDATA[
    {"name": "Alice", "age": 30, "items": ["a", "b"]}
    ]]>
  </task>
</poml>
```

## ネスト構造のルール

- `<poml>` はルート要素（他の要素の子にはなれない）
- `<role>`, `<task>`, `<let>`, `<example>`, `<output-format>`, `<output-schema>`, `<stylesheet>` は `<poml>` の直接の子
- `<example>` の中に `<input>` と `<output>` をネストできる
- `<document>`, `<table>`, `<img>` は `<task>` 内や他の要素内に配置可能

## バリデーション

```bash
# 構文チェック
poml check prompt.poml

# 詳細エラー表示
poml check --verbose prompt.poml
```

## ファイル命名規則

| ファイル | 用途 |
|---|---|
| `*.poml` | POML プロンプトファイル |
| `*.poml.vars` | 変数定義ファイル（オプション） |
