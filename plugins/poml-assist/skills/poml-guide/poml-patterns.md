# POML 設計パターン集

## パターン 1: テキスト要約

**用途**: 長文を指定の長さ・形式にまとめる

```xml
<poml>
  <role>
    あなたは優秀な編集者です。複雑な文章を分かりやすく整理することが得意です。
  </role>

  <let name="max_length">300</let>
  <let name="audience">一般読者</let>

  <task>
    以下のテキストを{{audience}}向けに、{{max_length}}文字以内で要約してください。
    専門用語は避け、重要なポイントを優先してください。
  </task>

  <example>
    <input>
      量子コンピュータは、量子力学の原理を利用した計算機です。従来のビットが0か1の
      どちらかの状態をとるのに対し、量子ビット（qubit）は重ね合わせにより0と1の
      両方の状態を同時にとることができます。
    </input>
    <output>
      量子コンピュータは、普通のコンピュータと違う仕組みで動く次世代の計算機です。
      特殊な「量子ビット」を使うことで、従来の何百万倍もの速さで複雑な計算ができます。
    </output>
  </example>

  <output-format>
    [要約文のみを出力。見出しや余分な説明は不要]
  </output-format>
</poml>
```

---

## パターン 2: テキスト分類

**用途**: 入力テキストをカテゴリに振り分ける

```xml
<poml>
  <task>
    以下のカスタマーサポートメッセージを分類してください。
  </task>

  <example>
    <input>注文した商品がまだ届いていません。</input>
    <output>shipping_inquiry</output>
  </example>

  <example>
    <input>返金をお願いしたいのですが。</input>
    <output>refund_request</output>
  </example>

  <example>
    <input>商品の使い方を教えてください。</input>
    <output>product_question</output>
  </example>

  <output-schema>
    {
      "type": "object",
      "properties": {
        "category": {
          "type": "string",
          "enum": ["shipping_inquiry", "refund_request", "product_question", "other"]
        },
        "confidence": {
          "type": "number",
          "minimum": 0,
          "maximum": 1
        }
      },
      "required": ["category", "confidence"]
    }
  </output-schema>
</poml>
```

---

## パターン 3: 構造化 JSON 出力

**用途**: テキストから構造化データを抽出する

```xml
<poml>
  <task>
    以下の求人情報から必要な情報を抽出してください。
  </task>

  <output-schema>
    {
      "type": "object",
      "properties": {
        "job_title": { "type": "string" },
        "company": { "type": "string" },
        "location": { "type": "string" },
        "salary_range": {
          "type": "object",
          "properties": {
            "min": { "type": "integer" },
            "max": { "type": "integer" },
            "currency": { "type": "string" }
          }
        },
        "required_skills": {
          "type": "array",
          "items": { "type": "string" }
        },
        "remote": { "type": "boolean" }
      },
      "required": ["job_title", "company"]
    }
  </output-schema>

  <output-format>
    JSON のみを出力する。コードブロックや説明文は不要。
  </output-format>
</poml>
```

---

## パターン 4: マルチターン会話（コンテキスト保持）

**用途**: 会話履歴を考慮した応答生成

```xml
<poml>
  <role>
    あなたは親切なカスタマーサポート担当者です。
    過去の会話の文脈を考慮して回答してください。
  </role>

  <let name="product_name">ProApp</let>
  <let name="company_name">株式会社Example</let>

  <task>
    {{company_name}}の{{product_name}}に関するサポート質問に対応してください。

    以下のルールに従ってください:
    1. 常に丁寧な敬語を使う
    2. 分からない場合は正直に伝え、エスカレーションを提案する
    3. 解決策は具体的なステップで説明する
  </task>

  <stylesheet>
    tone: polite
    language: ja
    format: plain
  </stylesheet>
</poml>
```

---

## パターン 5: コードレビュー

**用途**: コードの品質チェックと改善提案

```xml
<poml>
  <role>
    あなたはシニアソフトウェアエンジニアです。
    コードの品質、セキュリティ、パフォーマンスに精通しています。
  </role>

  <let name="language">Python</let>
  <let name="focus">セキュリティ</let>

  <task>
    以下の{{language}}コードを{{focus}}の観点でレビューしてください。
  </task>

  <output-format>
    ## レビュー結果

    ### 問題点
    | 重要度 | 行番号 | 問題 | 修正方法 |
    |---|---|---|---|

    ### 修正済みコード
    ```{{language}}
    [修正後のコード]
    ```

    ### 総評
    [全体的な評価]
  </output-format>
</poml>
```

---

## パターン 6: ドキュメント参照型 Q&A

**用途**: 特定のドキュメントを参照して質問に答える（RAG パターン）

```xml
<poml>
  <role>
    あなたは提供されたドキュメントのみを参照して回答するアシスタントです。
    ドキュメントに記載のない情報は「ドキュメントに情報がありません」と答えてください。
  </role>

  <task>
    以下のドキュメントを参照して、ユーザーの質問に答えてください。
    必ずドキュメントの内容に基づいて回答し、参照箇所を明示してください。
  </task>

  <document src="./knowledge_base.md" type="markdown" />

  <output-format>
    **回答**: [回答内容]

    **参照元**: [ドキュメントの該当セクション]
  </output-format>
</poml>
```

---

## パターン 7: 変数による動的カスタマイズ

**用途**: 同一テンプレートを複数のユースケースで再利用する

```xml
<poml>
  <let name="target_audience">マーケティング部門</let>
  <let name="report_type">月次売上</let>
  <let name="data_period">2024年1月</let>
  <let name="output_language">日本語</let>

  <role>
    あなたはビジネスアナリストです。データを分かりやすく解説することが得意です。
  </role>

  <task>
    {{data_period}}の{{report_type}}レポートを、{{target_audience}}向けに
    {{output_language}}で作成してください。

    含めるべき内容:
    - エグゼクティブサマリー（3文以内）
    - 主要KPIの分析
    - 前期比較
    - 次期に向けた提言
  </task>

  <table src="./monthly_data.csv" format="csv" />

  <stylesheet>
    tone: professional
    format: markdown
    sections: [summary, kpi, comparison, recommendations]
  </stylesheet>
</poml>
```

---

## パターン選択ガイド

| やりたいこと | 推奨パターン | 主要タグ |
|---|---|---|
| 長文を短くまとめる | パターン 1（要約） | `<task>`, `<example>`, `<output-format>` |
| テキストをカテゴリ分け | パターン 2（分類） | `<task>`, `<example>`, `<output-schema>` |
| データを JSON で取り出す | パターン 3（JSON 出力） | `<task>`, `<output-schema>` |
| チャットボットを作る | パターン 4（マルチターン） | `<role>`, `<task>`, `<let>`, `<stylesheet>` |
| コードを改善する | パターン 5（コードレビュー） | `<role>`, `<task>`, `<output-format>` |
| ドキュメントに基づく Q&A | パターン 6（RAG） | `<role>`, `<task>`, `<document>` |
| テンプレートを再利用 | パターン 7（変数化） | `<let>`, `<table>`, `<stylesheet>` |
