---
description: 開発コスト（トークン使用量・費用）を .dev-costs.json に記録する
---

# record-cost

このセッションの開発コストを記録します。`ccusage` でコスト情報を自動取得し、ユーザーから機能名を受け取って `.dev-costs.json` に保存してください。

## 手順

### 1. ccusage でコスト情報を取得

以下のコマンドを実行して、現在のセッションのコスト情報を取得してください:

```bash
npx ccusage@latest session --json --offline --since $(date +%Y%m%d)
```

出力の `sessions` 配列から、現在のプロジェクトに該当するセッションを特定する。複数ある場合は `lastActivity` が最新のものを使用する。

#### ccusage の出力から抽出するフィールド

| ccusage フィールド | 記録フィールド |
|---|---|
| `inputTokens` | `input_tokens` |
| `outputTokens` | `output_tokens` |
| `cacheReadTokens` | `cache_read_tokens` |
| `cacheCreationTokens` | `cache_creation_tokens` |
| `totalCost` | `total_cost_usd` |
| `modelsUsed[0]` | `model`（複数モデルの場合はカンマ区切り） |

### 2. 機能名とメタデータを入力

ユーザーに以下の情報を聞いてください:

- **機能名**（必須）: 何を実装・作業したかの簡潔な説明
- **作業時間（分）**（任意）: おおよその作業時間
- **変更ファイル数**（任意）: 変更したファイルの数
- **メモ**（任意）: 補足情報

```
以下の情報を教えてください:

1. 機能名（必須）: 何を実装しましたか？
2. 作業時間（任意、分単位）:
3. 変更ファイル数（任意）:
4. メモ（任意）:
```

### 3. 確認表示

ccusage から取得したデータとユーザー入力を合わせて確認してください:

```
## 記録内容の確認

| 項目 | 値 |
|---|---|
| 機能名 | {feature_name} |
| 日付 | {date} |
| セッションID | {session_id} |
| 作業時間 | {duration_minutes}分 |
| 変更ファイル数 | {files_changed} |
| 入力トークン | {input_tokens} |
| 出力トークン | {output_tokens} |
| キャッシュ読込 | {cache_read_tokens} |
| キャッシュ作成 | {cache_creation_tokens} |
| 合計コスト | ${total_cost_usd} |
| モデル | {model} |
| メモ | {memo} |

この内容で記録しますか？
```

### 4. `.dev-costs.json` に保存

ユーザーの確認後、プロジェクトルートの `.dev-costs.json` に追記してください。

**新規レコードの形式:**

```json
{
  "id": "UUID v4 形式",
  "feature_name": "機能名",
  "date": "YYYY-MM-DD",
  "session_id": "現在のセッションID",
  "duration_minutes": null,
  "files_changed": null,
  "tool_calls": null,
  "input_tokens": 150000,
  "output_tokens": 30000,
  "cache_read_tokens": 3508839,
  "cache_creation_tokens": 141434,
  "total_cost_usd": 1.25,
  "model": "claude-opus-4-6",
  "memo": null,
  "recorded_at": "ISO 8601 形式（タイムゾーン付き）"
}
```

**ファイル構造:**

```json
{
  "version": "1.0.0",
  "records": []
}
```

- ファイルが存在しない場合は新規作成する
- 既存ファイルがある場合は `records` 配列に追記する
- `id` は UUID v4 形式で生成する
- `recorded_at` は現在の ISO 8601 タイムスタンプ（タイムゾーン付き）

### 5. 完了メッセージ

```
コスト情報を .dev-costs.json に記録しました。

機能: {feature_name}
コスト: ${total_cost_usd}
```

初回作成時は以下も表示してください:

```
.dev-costs.json を新規作成しました。
.gitignore への追加を推奨します:
  echo '.dev-costs.json' >> .gitignore
```

## ccusage が使えない場合のフォールバック

`ccusage` の実行に失敗した場合は、ユーザーに `/cost` コマンドの出力を手動で共有してもらう:

```
ccusage でのコスト取得に失敗しました。
代わりに `/cost` コマンドの出力を貼り付けてください。
```

`/cost` 出力から以下をパースする:
- Input tokens, Output tokens, Cache read/write, Total cost, Model

## 重要な注意事項

- `ccusage` の出力を優先的に使用する（自動取得のため正確）
- 数値が取得できない場合は `null` として記録する
- セッションIDは環境から自動取得する（ユーザーに入力させない）
- 金額は USD で記録する
