use claude_code_hooks::prelude::*;

use crate::code_improve::run_format_and_lint;
use crate::command_check::is_dangerous_command;
use crate::hook_specific_output::HookSpecificOutput;
use crate::logging::log_to_file;
use crate::mac_notification::send_mac_notification;
use crate::user_prompt::{UserPromptSubmitResponse, evaluate_prompt};

mod mac_notification;
mod user_prompt;
mod hook_specific_output;
mod logging;
mod language_check;
mod code_improve;
mod command_check;

#[tokio::main]
async fn main() -> Result<(), HookError> {
    let hook = HookRunner::new()
        // プロンプト送信時のフック
        .on_user_prompt_submit_sync(|event| {
            let score = evaluate_prompt(&event.prompt);
            const THRESHOLD: i32 = 60;

            let notification_message = format!("プロンプト評価スコア: {score}/100");
            log_to_file("PROMPT_SUBMIT", &format!("スコア: {}, プロンプト: {}", score, &event.prompt[..50.min(event.prompt.len())]));

            if score < THRESHOLD {
                send_mac_notification(
                    "Claude Code - プロンプト品質警告",
                    "品質スコアが低すぎます",
                    &notification_message
                );

                println!("{}", serde_json::to_string_pretty(&UserPromptSubmitResponse {
                    decision: Some("block".to_string()),
                    reason: Some(format!("プロンプト評価スコアが低すぎます ({score})")),
                    hook_specific_output: Some(HookSpecificOutput {
                        hook_event_name: "UserPromptSubmit".to_string(),
                        additional_context: notification_message,
                    }),
                    continue_: false,
                    stop_reason: Some("プロンプトの品質が基準を満たしていません".to_string()),
                    suppress_output: Some(true),
                }).unwrap());
            } else {
                send_mac_notification(
                    "Claude Code - プロンプト受信",
                    "品質チェック完了",
                    &notification_message
                );

                println!("{}", serde_json::to_string_pretty(&UserPromptSubmitResponse {
                    decision: None,
                    reason: None,
                    hook_specific_output: Some(HookSpecificOutput {
                        hook_event_name: "UserPromptSubmit".to_string(),
                        additional_context: notification_message,
                    }),
                    continue_: true,
                    stop_reason: None,
                    suppress_output: Some(false),
                }).unwrap());
            }

            Ok(())
        })

        // ツール使用前のフック（権限確認）
        .on_pre_tool_use_sync(|event| {
            let tool_info = format!("ツール: {}", event.tool_name);
            log_to_file("PRE_TOOL_USE", &tool_info);

            // 危険なコマンドの場合は確認通知を送信
            if is_dangerous_command(&event.tool_name, &format!("{:?}", event.tool_input)) {
                send_mac_notification(
                    "Claude Code - 危険な操作",
                    &format!("{}の使用", event.tool_name),
                    "危険な可能性のあるコマンドが実行されようとしています"
                );

                log_to_file("DANGEROUS_COMMAND", &format!("{}: {:?}", event.tool_name, event.tool_input));
            } else {
                send_mac_notification(
                    "Claude Code - ツール使用開始",
                    &event.tool_name,
                    "ツールの使用を開始します"
                );
            }

            Ok(())
        })

        // ツール使用後のフック（結果通知とフォーマット実行）
        .on_post_tool_use_sync(|event| {
            let success_msg = format!("ツール '{}' の実行が完了しました", event.tool_name);
            log_to_file("POST_TOOL_USE", &success_msg);

            // ファイル編集系ツールの場合は自動フォーマット/リントを実行
            match event.tool_name.as_str() {
                "Edit" | "MultiEdit" | "Write" => {
                    send_mac_notification(
                        "Claude Code - ファイル編集完了",
                        &event.tool_name,
                        &success_msg
                    );

                    // 自動フォーマット/リントを実行
                    match run_format_and_lint() {
                        Ok(_) => {
                            log_to_file("AUTO_FORMAT", "フォーマット/リント成功");
                        }
                        Err(e) => {
                            log_to_file("AUTO_FORMAT_ERROR", &e);
                        }
                    }
                }
                _ => {
                    send_mac_notification(
                        "Claude Code - ツール実行完了",
                        &event.tool_name,
                        &success_msg
                    );
                }
            }

            Ok(())
        })

        // セッション開始時のフック (削除: APIが存在しない)
        // .on_session_start_sync は利用できません

        // セッション終了時のフック (削除: APIが存在しない)
        // .on_session_end_sync は利用できません
        ;

    hook.run_from_stdin().await
}
