# 🧨 Удалить GHCR-образ через интерактивный Rust-скрипт
ghcr-clean:
    @echo "🧹 Запуск ghcr-cleaner.rs через cargo-script"
    rust-script scripts/ghcr-cleaner.rs


# 🧨 Удалить GHCR-образ через интерактивный Rust-скрипт
ghcr-list:
    @echo "🧹 Запуск ghcr-cleaner.rs через cargo-script"
    rust-script scripts/gchr/list.rs

ghcr-delete-old DAYS:
    if [ -z "{{DAYS}}" ]; then DAYS=30; fi
    @echo "🧹 Удаление GHCR тегов старше $$DAYS дней"
    rust-script scripts/ghcr-delete-old.rs -- $$DAYS