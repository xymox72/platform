use chrono::Utc;
use dialoguer::Confirm;

use crate::github::config::{get_user, get_packages, get_versions};


pub fn run(days: i64) {
    let user = get_user();
    let now = Utc::now();

    for pkg in get_packages() {
        println!("\n📦 Образ: {}", pkg.name);

        for version in get_versions(&pkg.name) {
            let tags = &version.metadata.container.tags;

            if tags.iter().any(|t| ["latest", "main", "stable"].contains(&t.as_str())) {
                continue;
            }

            let age = now.signed_duration_since(version.updated_at).num_days();
            if age < days {
                continue;
            }

            let tag_display = tags.join(", ");
            println!("📅 {} ({} дней назад)", tag_display, age);

            if Confirm::new()
                .with_prompt(format!("🗑 Удалить тег(и) '{}'? (старше {} дней)", tag_display, days))
                .default(false)
                .interact()
                .unwrap()
            {
                let url = format!(
                    "/users/{}/packages/container/{}/versions/{}",
                    user, pkg.name, version.id
                );

                let status = std::process::Command::new("gh")
                    .args(["api", "-X", "DELETE", "-H", "Accept: application/vnd.github.v3+json", &url])
                    .status()
                    .expect("❌ Не удалось удалить");

                if status.success() {
                    println!("✅ Удалено: {}", tag_display);
                } else {
                    println!("❌ Ошибка при удалении: {}", tag_display);
                }
            }
        }
    }
}
