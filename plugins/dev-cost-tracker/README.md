# dev-cost-tracker

Claude Code の開発コスト（トークン使用量・費用）を機能単位で記録するプラグイン。

## 機能

- **セッション終了時のリマインダー**: Stop フックでコスト未記録のセッションをリマインド
- **`/record-cost` コマンド**: `/cost` の出力を元に `.dev-costs.json` へコスト情報を記録
- **コスト分析スキル**: 記録されたデータの分析・レポート作成をサポート

## インストール

```bash
claude plugins add ./plugins/dev-cost-tracker
```

## 使い方

### コストの記録

1. 作業が完了したら `/cost` を実行してコスト情報を確認
2. `/record-cost` を実行
3. `/cost` の出力を貼り付け
4. 機能名を入力（作業時間・メモは任意）
5. 確認して保存

### データの確認

`.dev-costs.json` に JSON 形式で記録される。コスト分析や見積もりの参考に利用できる。

## ファイル構成

```
dev-cost-tracker/
├── .claude-plugin/
│   └── plugin.json          # プラグインマニフェスト
├── hooks/
│   ├── hooks.json            # フック定義
│   └── scripts/
│       ├── stop_reminder.py      # Stop フックスクリプト
│       └── test_stop_reminder.py # テスト
├── commands/
│   └── dev-cost-tracker/
│       └── record-cost.md    # /record-cost コマンド
├── skills/
│   └── dev-cost-tracker/
│       └── SKILL.md          # コスト分析スキル
└── README.md
```

## ライセンス

MIT
