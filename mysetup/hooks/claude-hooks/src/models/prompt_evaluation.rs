use serde::{Deserialize, Serialize};
use std::path::Path;

/// Hook-specific output structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HookSpecificOutput {
    #[serde(rename = "hookEventName")]
    pub hook_event_name: String,
    #[serde(rename = "additionalContext")]
    pub additional_context: String,
}

/// User prompt submit response
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Prompt quality evaluator
pub struct PromptEvaluator {
    threshold: i32,
}

impl Default for PromptEvaluator {
    fn default() -> Self {
        Self { threshold: 60 }
    }
}

impl PromptEvaluator {
    pub fn new(threshold: i32) -> Self {
        Self { threshold }
    }

    /// Evaluate prompt quality and return a score
    pub fn evaluate(&self, prompt: &str) -> i32 {
        let mut score = 100;

        // Slash commands get maximum score
        if prompt.trim_start().starts_with('/') {
            return 100;
        }

        // Check length
        let char_count = prompt.chars().count();
        if char_count > 500 || char_count < 15 {
            score -= 30;
        }

        // Check for vague expressions
        score -= self.check_vague_words(prompt);

        // Check for constraints
        if !self.has_constraints(prompt) {
            score -= 20;
        }

        // Check for language-specific files
        if !check_language_files_exist() {
            score -= 15;
        }

        // Check for non-coding intent
        score -= self.check_non_coding_intent(prompt);

        // Check for specific task verbs
        if !self.has_specific_task_verb(prompt) {
            score -= 10;
        }

        // Security and ethical concerns
        if self.has_definite_block_keywords(prompt) {
            return 0;
        }

        score -= self.check_sensitive_security_keywords(prompt);

        score.max(0)
    }

    /// Check if the prompt passes the quality threshold
    pub fn passes_threshold(&self, prompt: &str) -> bool {
        self.evaluate(prompt) >= self.threshold
    }

    fn check_vague_words(&self, prompt: &str) -> i32 {
        let vague_words = [
            "なんでも", "適当に", "よろしく", "問題を解決",
            "動くようにして", "コードをなんとかして", "良い感じにして",
        ];
        vague_words.iter()
            .filter(|&w| prompt.contains(w))
            .count() as i32 * 15
    }

    fn has_constraints(&self, prompt: &str) -> bool {
        prompt.contains("行以内") || prompt.contains("diff") || prompt.contains("差分")
    }

    fn check_non_coding_intent(&self, prompt: &str) -> i32 {
        let non_coding_keywords = [
            "詩", "物語", "エッセイ", "マーケティング",
            "一般的な知識", "歴史", "レシピ", "小説",
        ];
        non_coding_keywords.iter()
            .filter(|&w| prompt.contains(w))
            .count() as i32 * 20
    }

    fn has_specific_task_verb(&self, prompt: &str) -> bool {
        let task_verbs = [
            "実装", "作成", "デバッグ", "リファクタリング",
            "テスト", "最適化", "修正", "生成",
        ];
        task_verbs.iter().any(|&w| prompt.contains(w))
    }

    fn has_definite_block_keywords(&self, prompt: &str) -> bool {
        let block_keywords = [
            "ヘイトスピーチ", "差別", "テロ", "児童ポルノ",
            "違法行為", "詐欺",
        ];
        block_keywords.iter().any(|&w| prompt.contains(w))
    }

    fn check_sensitive_security_keywords(&self, prompt: &str) -> i32 {
        let security_keywords = [
            "マルウェア", "ハッキング", "攻撃", "脆弱性", "エクスプロイト",
        ];
        security_keywords.iter()
            .filter(|&w| prompt.contains(w))
            .count() as i32 * 25
    }
}

/// Check if language-specific project files exist
fn check_language_files_exist() -> bool {
    let candidates = [
        "Cargo.toml", "package.json", "requirements.txt", "Pipfile",
        "pyproject.toml", "Gemfile", "go.mod", "pom.xml",
        "build.gradle", "composer.json",
    ];
    candidates.iter().any(|f| Path::new(f).exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slash_command() {
        let evaluator = PromptEvaluator::default();
        assert_eq!(evaluator.evaluate("/help"), 100);
    }

    #[test]
    fn test_vague_prompt() {
        let evaluator = PromptEvaluator::default();
        let score = evaluator.evaluate("なんでも動くようにして");
        assert!(score < 60);
    }

    #[test]
    fn test_specific_prompt() {
        let evaluator = PromptEvaluator::default();
        let score = evaluator.evaluate("Rustで認証機能を実装してください。30行以内で記述してください。");
        assert!(score >= 60);
    }
}
