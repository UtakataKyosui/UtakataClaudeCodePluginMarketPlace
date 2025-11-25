use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::formatter::ProjectType;

/// Linter for various project types
pub struct Linter;

impl Linter {
    /// Run linter for all detected project types
    pub fn run_all() -> Vec<LintResult> {
        let mut results = Vec::new();

        let project_type = ProjectType::detect();
        match project_type {
            ProjectType::Rust => {
                results.extend(Self::lint_rust());
            }
            ProjectType::NodeJs => {
                results.extend(Self::lint_nodejs());
            }
            ProjectType::Python => {
                results.extend(Self::lint_python());
            }
            ProjectType::Unknown => {
                results.push(LintResult::skipped("No recognized project type found"));
            }
        }

        results
    }

    /// Lint specific file
    pub fn lint_file(file_path: &Path) -> Vec<LintResult> {
        let extension = file_path.extension().and_then(|s| s.to_str());

        match extension {
            Some("rs") => Self::lint_rust_file(file_path),
            Some("js") | Some("jsx") | Some("ts") | Some("tsx") => Self::lint_js_file(file_path),
            Some("py") => Self::lint_python_file(file_path),
            _ => vec![LintResult::skipped(&format!(
                "Unsupported file type: {:?}",
                file_path
            ))],
        }
    }

    fn lint_rust() -> Vec<LintResult> {
        vec![Self::run_cargo_clippy()]
    }

    fn lint_rust_file(file_path: &Path) -> Vec<LintResult> {
        match find_cargo_project_root(file_path) {
            Ok(root) => vec![Self::run_cargo_clippy_in_dir(&root)],
            Err(_) => vec![LintResult::skipped("No Cargo.toml found")],
        }
    }

    fn run_cargo_clippy() -> LintResult {
        match Command::new("cargo")
            .args(["clippy", "--", "-W", "clippy::all"])
            .output()
        {
            Ok(output) if output.status.success() => {
                LintResult::success("cargo clippy", "No warnings found")
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("could not compile") {
                    LintResult::warning("cargo clippy", &stderr)
                } else {
                    LintResult::error("cargo clippy", &stderr)
                }
            }
            Err(e) => LintResult::error("cargo clippy", &e.to_string()),
        }
    }

    fn run_cargo_clippy_in_dir(project_root: &Path) -> LintResult {
        match Command::new("cargo")
            .args(["clippy", "--", "-W", "clippy::all"])
            .current_dir(project_root)
            .output()
        {
            Ok(output) if output.status.success() => {
                LintResult::success("cargo clippy", "No warnings found")
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("could not compile") {
                    LintResult::warning("cargo clippy", &stderr)
                } else {
                    LintResult::error("cargo clippy", &stderr)
                }
            }
            Err(e) => LintResult::error("cargo clippy", &e.to_string()),
        }
    }

    fn lint_nodejs() -> Vec<LintResult> {
        let mut results = Vec::new();

        // Try eslint
        match Command::new("npx").args(["eslint", ".", "--fix"]).output() {
            Ok(output) if output.status.success() => {
                results.push(LintResult::success("eslint", "No issues found"));
            }
            Ok(output) => {
                results.push(LintResult::warning(
                    "eslint",
                    &String::from_utf8_lossy(&output.stderr),
                ));
            }
            Err(_) => {
                // Try npm run lint
                match Command::new("npm").args(["run", "lint"]).output() {
                    Ok(output) if output.status.success() => {
                        results.push(LintResult::success("npm run lint", "No issues found"));
                    }
                    _ => results.push(LintResult::not_available("eslint or npm run lint")),
                }
            }
        }

        results
    }

    fn lint_js_file(_file_path: &Path) -> Vec<LintResult> {
        Self::lint_nodejs()
    }

    fn lint_python() -> Vec<LintResult> {
        match Command::new("flake8").args(["."]).output() {
            Ok(output) if output.status.success() => {
                vec![LintResult::success("flake8", "No issues found")]
            }
            Ok(output) => {
                vec![LintResult::warning(
                    "flake8",
                    &String::from_utf8_lossy(&output.stderr),
                )]
            }
            Err(_) => vec![LintResult::not_available("flake8")],
        }
    }

    fn lint_python_file(file_path: &Path) -> Vec<LintResult> {
        match Command::new("flake8").arg(file_path).output() {
            Ok(output) if output.status.success() => {
                vec![LintResult::success(
                    "flake8",
                    &format!("No issues in: {:?}", file_path),
                )]
            }
            Ok(output) => {
                vec![LintResult::warning(
                    "flake8",
                    &String::from_utf8_lossy(&output.stderr),
                )]
            }
            Err(_) => vec![LintResult::not_available("flake8")],
        }
    }
}

/// Lint result
#[derive(Debug, Clone)]
pub struct LintResult {
    pub tool: String,
    pub status: LintStatus,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintStatus {
    Success,
    Warning,
    Error,
    NotAvailable,
    Skipped,
}

impl LintResult {
    pub fn success(tool: &str, message: &str) -> Self {
        Self {
            tool: tool.to_string(),
            status: LintStatus::Success,
            message: message.to_string(),
        }
    }

    pub fn warning(tool: &str, message: &str) -> Self {
        Self {
            tool: tool.to_string(),
            status: LintStatus::Warning,
            message: message.to_string(),
        }
    }

    pub fn error(tool: &str, message: &str) -> Self {
        Self {
            tool: tool.to_string(),
            status: LintStatus::Error,
            message: message.to_string(),
        }
    }

    pub fn not_available(tool: &str) -> Self {
        Self {
            tool: tool.to_string(),
            status: LintStatus::NotAvailable,
            message: format!("{} is not available", tool),
        }
    }

    pub fn skipped(message: &str) -> Self {
        Self {
            tool: "".to_string(),
            status: LintStatus::Skipped,
            message: message.to_string(),
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self.status {
            LintStatus::Success => "✅",
            LintStatus::Warning => "⚠️",
            LintStatus::Error => "❌",
            LintStatus::NotAvailable => "⚠️",
            LintStatus::Skipped => "ℹ️",
        }
    }

    pub fn print(&self) {
        println!("  {} {}: {}", self.emoji(), self.tool, self.message);
    }
}

/// Find Cargo project root
fn find_cargo_project_root(file_path: &Path) -> Result<PathBuf> {
    let mut current_dir = file_path.parent().unwrap_or(file_path);

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(current_dir.to_path_buf());
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => return Err(anyhow::anyhow!("No Cargo.toml found in parent directories")),
        }
    }
}
