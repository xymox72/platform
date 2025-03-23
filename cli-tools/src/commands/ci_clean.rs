use crate::github::env::repo;

use crate::github::{gh_json, gh_status};


use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Artifact {
    id: u64,
    name: String,
    size_in_bytes: u64,
}

#[derive(Debug, Deserialize)]
struct ArtifactList {
    artifacts: Vec<Artifact>,
}

pub fn run(repo_arg: &str) {
    let repo = if !repo_arg.is_empty() {
        repo_arg.to_string()
    } else if let Some(env_repo) = repo() {
        env_repo
    } else {
        eprintln!("❌ Укажи репозиторий: --repo или переменную GITHUB_REPOSITORY в .env");
        std::process::exit(1);
    };
    

    println!("🔎 Чистим CI артефакты из репозитория: {}", repo);

    let endpoint = format!("/repos/{}/actions/artifacts", repo);
    let data: ArtifactList = gh_json(&["api", &endpoint]);

    if data.artifacts.is_empty() {
        println!("✅ Нет CI артефактов");
        return;
    }

    println!("📦 Найдено {} артефактов:", data.artifacts.len());

    for artifact in data.artifacts {
        println!("  🧱 {} ({:.2} MB)", artifact.name, artifact.size_in_bytes as f64 / 1_048_576.0);

        let del_url = format!("/repos/{}/actions/artifacts/{}", repo, artifact.id);
        let success = gh_status(&["api", "-X", "DELETE", &del_url]);

        if success {
            println!("    ✅ Удалён: {}", artifact.name);
        } else {
            println!("    ❌ Не удалось удалить: {}", artifact.name);
        }
    }

    println!("🏁 Готово");
}
