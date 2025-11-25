use serde::{Deserialize, Serialize};

use crate::{hook_specific_output::HookSpecificOutput, language_check::check_language_files_exist};


#[derive(Serialize, Deserialize)]
pub struct UserPromptSubmitResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hookSpecificOutput")]
    pub hook_specific_output: Option<HookSpecificOutput>,
    #[serde(rename = "continue")]
    pub continue_: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stopReason")]
    pub stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "suppressOutput")]
    pub suppress_output: Option<bool>,
}

/// プロンプトの品質を評価する関数
pub fn evaluate_prompt(prompt: &str) -> i32 {
    let mut score = 100;

    // スラッシュコマンドの場合は最高スコアを付与
    if prompt.trim_start().starts_with('/') {
        return 100;
    }

    // 文字数制限
    if prompt.chars().count() > 500 || prompt.chars().count() < 15 {
        score -= 30;
    }

    // 曖昧な表現チェック
    let vague_words = [
        "なんでも",
        "適当に",
        "よろしく",
        "問題を解決",
        "動くようにして",
        "コードをなんとかして",
        "良い感じにして",
    ];
    for w in vague_words.iter() {
        if prompt.contains(w) {
            score -= 15;
        }
    }

    // 制約条件チェック
    if !(prompt.contains("行以内") || prompt.contains("diff") || prompt.contains("差分")) {
        score -= 20;
    }

    // 言語判定
    if !check_language_files_exist() {
        score -= 15;
    }

    // 非プログラミング関連の意図
    let non_coding_intent_keywords = [
        "詩",
        "物語",
        "エッセイ",
        "マーケティング",
        "一般的な知識",
        "歴史",
        "レシピ",
        "小説",
    ];
    for w in non_coding_intent_keywords.iter() {
        if prompt.contains(w) {
            score -= 20;
        }
    }

    // 具体的なタスク動詞の有無
    let specific_task_verbs = [
        "実装",
        "作成",
        "デバッグ",
        "リファクタリング",
        "テスト",
        "最適化",
        "修正",
        "生成",
    ];
    let has_specific_verb = specific_task_verbs.iter().any(|w| prompt.contains(w));
    if !has_specific_verb {
        score -= 10;
    }

    // 安全性・倫理的な懸念
    let definite_block_keywords = [
        "ヘイトスピーチ",
        "差別",
        "テロ",
        "児童ポルノ",
        "違法行為",
        "詐欺",
    ];
    for w in definite_block_keywords.iter() {
        if prompt.contains(w) {
            return 0;
        }
    }

    let sensitive_security_keywords = [
        "マルウェア",
        "ハッキング",
        "攻撃",
        "脆弱性",
        "エクスプロイト",
    ];
    for w in sensitive_security_keywords.iter() {
        if prompt.contains(w) {
            score -= 25;
        }
    }

    score.max(0)
}