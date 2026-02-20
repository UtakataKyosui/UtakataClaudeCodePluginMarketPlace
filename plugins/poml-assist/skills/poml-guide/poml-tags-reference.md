# POML タグリファレンス

## `<poml>`

ルート要素。すべての POML ファイルはこのタグで囲む。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `version` | string | POML バージョン（省略可、例: `"1.0"`） |

```xml
<poml version="1.0">
  <!-- 内容 -->
</poml>
```

---

## `<role>`

AI エージェントのペルソナ・専門性・トーンを定義する。

**属性**: なし

**使用例**:

```xml
<poml>
  <role>
    あなたはシニアソフトウェアエンジニアです。
    10年以上のバックエンド開発経験を持ち、コードの品質と保守性を重視します。
    技術的に正確で、簡潔な説明を心がけてください。
  </role>
  <task>以下のコードをレビューしてください。</task>
</poml>
```

---

## `<task>`

実行するタスクを定義する。**必須タグ**。

**属性**: なし

**使用例**:

```xml
<poml>
  <task>
    以下の要件を満たすコードレビューを実施してください:
    1. バグの可能性
    2. セキュリティ問題
    3. パフォーマンス改善点
    4. コーディング規約への準拠
  </task>
</poml>
```

---

## `<example>`

Few-shot 学習用のサンプルを提供する。`<input>` と `<output>` をペアで定義。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `type` | string | サンプルの種別（`positive` / `negative` など） |

**使用例**:

```xml
<poml>
  <task>テキストの感情を分類してください。</task>

  <example>
    <input>今日は天気が良くて気持ちいい！</input>
    <output>positive</output>
  </example>

  <example>
    <input>雨が続いて憂鬱です。</input>
    <output>negative</output>
  </example>

  <example type="negative">
    <input>明日の会議の資料を準備した。</input>
    <output>neutral</output>
  </example>
</poml>
```

---

## `<output-format>`

期待する出力の形式・構造を指定する。

**属性**: なし

**使用例**:

```xml
<poml>
  <task>記事を要約してください。</task>

  <output-format>
    以下の形式で出力してください:

    ## タイトル
    [記事のタイトル]

    ## 要点
    - [要点1]
    - [要点2]
    - [要点3]

    ## 結論
    [1-2文でまとめ]
  </output-format>
</poml>
```

---

## `<document>`

外部ドキュメントや長文テキストをプロンプトに埋め込む。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `src` | string | ファイルパス（相対パスまたは絶対パス） |
| `type` | string | コンテンツ種別（`text`, `markdown`, `code`, `json` など） |

**使用例**:

```xml
<poml>
  <task>以下のドキュメントを要約してください。</task>

  <!-- ファイルから読み込み -->
  <document src="./report.md" type="markdown" />

  <!-- インラインで定義 -->
  <document type="text">
    ここに長い文章を直接書くこともできます。
    複数行にわたる長文テキストの場合に便利です。
  </document>
</poml>
```

---

## `<table>`

表形式のデータをプロンプトに含める。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `src` | string | CSV/TSV ファイルパス |
| `format` | string | フォーマット種別（`csv`, `tsv`, `markdown`） |

**使用例**:

```xml
<poml>
  <task>以下の売上データを分析してください。</task>

  <table src="./sales_data.csv" format="csv" />

  <!-- インライン Markdown テーブル -->
  <table format="markdown">
    | 月 | 売上 | 前月比 |
    |---|---|---|
    | 1月 | 100万円 | - |
    | 2月 | 120万円 | +20% |
    | 3月 | 110万円 | -8% |
  </table>
</poml>
```

---

## `<img>`

画像ファイルを参照する（マルチモーダルモデル用）。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `src` | string | 画像ファイルパスまたは URL |
| `alt` | string | 代替テキスト（アクセシビリティ・フォールバック） |

**使用例**:

```xml
<poml>
  <task>以下の画像を説明してください。</task>

  <img src="./screenshot.png" alt="アプリケーションのスクリーンショット" />
</poml>
```

---

## `<stylesheet>`

出力のフォーマット・スタイルを宣言的に制御する。

**属性**: なし

詳細は [poml-stylesheet.md](./poml-stylesheet.md) を参照。

**使用例**:

```xml
<poml>
  <stylesheet>
    tone: professional
    language: ja
    length: concise
    format: markdown
  </stylesheet>

  <task>製品の特徴を説明してください。</task>
</poml>
```

---

## `<output-schema>`

JSON Schema を使って出力の構造を厳密に定義する。JSON 出力が必要な場合に使用。

**属性**: なし

**使用例**:

```xml
<poml>
  <task>以下のテキストから情報を抽出してください。</task>

  <output-schema>
    {
      "type": "object",
      "properties": {
        "name": { "type": "string", "description": "人物の名前" },
        "age": { "type": "integer", "description": "年齢" },
        "skills": {
          "type": "array",
          "items": { "type": "string" },
          "description": "スキル一覧"
        }
      },
      "required": ["name"]
    }
  </output-schema>
</poml>
```

---

## `<let>`

再利用可能な変数を定義する。`{{variable_name}}` で参照。

**属性**:
| 属性 | 型 | 説明 |
|---|---|---|
| `name` | string | 変数名（必須） |

**使用例**:

```xml
<poml>
  <let name="language">日本語</let>
  <let name="tone">丁寧</let>
  <let name="max_words">200</let>

  <task>
    {{language}}で、{{tone}}なトーンを使い、
    {{max_words}}語以内でメールの返信を作成してください。
  </task>
</poml>
```
