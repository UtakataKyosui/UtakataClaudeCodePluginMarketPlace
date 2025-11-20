---
name: speckit-skilled
descripton: 機能実装や修正時に仕様を策定する際に使用する。
---

# SpecKit Skilled

仕様駆動開発ツール「Speckit」をAgent Skillとして調整したもの。
```sh
╭──────────────────────────────────────────────────────────────── Next Steps ─────────────────────────────────────────────────────────────────╮
│                                                                                                                                             │
│  1. You're already in the project directory!                                                                                                │
│  2. Start using slash commands with your AI agent:                                                                                          │
│     2.1 /speckit.constitution - Establish project principles                                                                                │
│     2.2 /speckit.specify - Create baseline specification                                                                                    │
│     2.3 /speckit.plan - Create implementation plan                                                                                          │
│     2.4 /speckit.tasks - Generate actionable tasks                                                                                          │
│     2.5 /speckit.implement - Execute implementation                                                                                         │
│                                                                                                                                             │
╰─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯

╭─────────────────────────────────────────────────────────── Enhancement Commands ────────────────────────────────────────────────────────────╮
│                                                                                                                                             │
│  Optional commands that you can use for your specs (improve quality & confidence)                                                           │
│                                                                                                                                             │
│  ○ /speckit.clarify (optional) - Ask structured questions to de-risk ambiguous areas before planning (run before /speckit.plan if used)     │
│  ○ /speckit.analyze (optional) - Cross-artifact consistency & alignment report (after /speckit.tasks, before /speckit.implement)            │
│  ○ /speckit.checklist (optional) - Generate quality checklists to validate requirements completeness, clarity, and consistency (after       │
│  /speckit.plan)                                                                                                                             │
│                                                                                                                                             │
╰─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

これらの機能をAgent Skillとして動作させる

