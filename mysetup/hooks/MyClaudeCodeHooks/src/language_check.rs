use std::path::Path;

/// プロジェクトの言語固有ファイルが存在するかをチェック
pub fn check_language_files_exist() -> bool {
    let candidates = [
        "Cargo.toml",
        "package.json",
        "requirements.txt",
        "Pipfile",
        "pyproject.toml",
        "Gemfile",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "composer.json",
    ];
    candidates.iter().any(|f| Path::new(f).exists())
}