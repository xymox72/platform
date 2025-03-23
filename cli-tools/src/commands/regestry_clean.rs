use crate::github::{
    self,
    config::{get_packages_by_type, get_versions_by_type, gh_status, get_project_scope},
};

pub fn run(package_type: String, name_filter: Option<String>, all: bool) {
    // Проверка: если ни --name, ни --all — используем PROJECT_SCOPE
    let scope = get_project_scope();

    let packages = get_packages_by_type(&package_type);
    if packages.is_empty() {
        println!("✅ Нет пакетов типа `{}` in scope {} в GitHub Packages.", package_type, scope);
        return;
    }

    for pkg in packages {
        let should_delete = match &name_filter {
            Some(name) => pkg.name == *name,
            None => {
                if all {
                    true
                } else {
                    pkg.name.contains(&scope)
                }
            }
        };

        if !should_delete {
            continue;
        }

        println!("🧨 Удаляем {} пакет: {}", package_type, pkg.name);

        let safe_name = pkg.name.replace("/", "%2f");
        let versions = get_versions_by_type(&package_type, &safe_name);
        println!("{:?}", versions);
        for version in versions {
            let tags = version.metadata.container.tags.join(", ");
            println!("  🗑 Удаляем версию {} (tags: {})", version.id, tags);

            let ok = gh_status(&[
                "api",
                &format!(
                    "/users/{}/packages/{}/{}/versions/{}",
                    github::env::username(),
                    package_type,
                    safe_name,
                    version.id
                ),
                "-X",
                "DELETE",
            ]);

            if ok {
                println!("    ✅ Удалено");
            } else {
                println!("    ❌ Ошибка при удалении");
            }
        }
    }

    println!("✅ Завершено.");
}
