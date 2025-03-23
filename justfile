# === Rust CLI: cli-tools ===

ghcr-list:
    @echo "📦 Список GHCR пакетов"
    cargo run --manifest-path cli-tools/Cargo.toml -- list

ghcr-delete-old DAYS:
    @echo "🧹 Удаление GHCR тегов старше {{DAYS}} дней"
    cargo run --manifest-path cli-tools/Cargo.toml -- delete-old --days {{DAYS}}


ci-clean:
    cargo run --manifest-path cli-tools/Cargo.toml -- ci-clean


# === Сборка и тесты cli-tools ===

build:
    cargo build --manifest-path cli-tools/Cargo.toml

test:
    cargo test --manifest-path cli-tools/Cargo.toml

install:
    cargo install --path cli-tools


# === Очистка ===

clean-cargo:
    @echo "🧼 Удаляем target и .cargo/script-cache"
    cargo clean --manifest-path cli-tools/Cargo.toml
    rm -rf ~/.cargo/script-cache || true
    rm -rf cli-tools/.rust-script || true
    rm -rf cli-tools/target || true

clean-files:
    @echo "🧽 Удаляем временные .lock/.tmp/.bak/.old"
    find . -type f \( \
        -name "*.lock" -o \
        -name "*.tmp"  -o \
        -name "*.bak"  -o \
        -name "*.old" \
    \) -exec rm -v {} \; || true

clean-all:
    just clean-cargo
    just clean-files

    @echo "🧹 Останавливаем все контейнеры..."
    docker stop $(docker ps -aq) || true

    @echo "🗑 Удаляем все контейнеры..."
    docker rm -f $(docker ps -aq) || true

    @echo "🖼 Удаляем все образы..."
    docker rmi -f $(docker images -q) || true

    @echo "📦 Удаляем все тома..."
    docker volume rm $(docker volume ls -q) || true

    @echo "🌐 Удаляем все кастомные сети..."
    docker network rm $(docker network ls -q | grep -v "bridge\|host\|none") || true

    @echo "🧼 Чистим builder-кэш..."
    docker builder prune -a -f

    @echo "✅ Docker окружение очищено."
