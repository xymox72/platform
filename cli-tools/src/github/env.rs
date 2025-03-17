use std::env;
use crate::github::gh_json;

/// Загружаем .env при первом обращении
fn init() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        match dotenvy::dotenv() {
            Ok(path) => println!("📄 .env загружен из: {:?}", path),
            Err(err) => eprintln!("⚠️ .env не найден: {}", err),
        }
    });
}

/// Получить GitHub username
pub fn username() -> String {
    init();

    if let Ok(user) = env::var("GITHUB_USERNAME") {
        return user;
    }

    // Fallback через GitHub CLI
    let json: serde_json::Value = gh_json(&["api", "user"]);
    let login = json["login"]
        .as_str()
        .expect("❌ Не удалось получить имя пользователя");


    login.to_string()
}

/// Получить токен GitHub
pub fn token() -> String {
    init();

    if let Ok(token) = env::var("GITHUB_TOKEN") {

        return token;
    }

    let output = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output()
        .expect("❌ Не удалось запустить `gh auth token`");

    if output.status.success() {
        let token = String::from_utf8_lossy(&output.stdout).trim().to_string();

        token
    } else {
        eprintln!("❌ Не удалось получить токен: переменная GITHUB_TOKEN не установлена и `gh auth token` не сработал");
        std::process::exit(1);
    }
}

/// Получить имя репозитория (owner/name)
pub fn repo() -> Option<String> {
    init();

    if let Ok(repo) = env::var("GITHUB_REPOSITORY") {
        Some(repo)
    } else {
        None
    }
}
