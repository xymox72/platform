# 🛠️ DevOps-папка проекта

Здесь хранятся все инструменты и конфигурации, связанные с DevOps, окружением, сборкой, CI/CD и разработкой в контейнерах.

---

## 📦 Dockerfile'ы

| Файл                    | Назначение                                               |
|-------------------------|----------------------------------------------------------|
| `Dockerfile.dev-base`   | Базовый образ: Alpine + `curl`, `git`, `bash`, `build`-инструменты |
| `Dockerfile.dev-rust`   | Rust-ориентированный образ на базе `dev-base` + `rustup`, `cargo`, `sccache`, `clippy`, `rustfmt` |
| `Dockerfile.dev-python` | Образ для Python-утилит, аналитики, скриптов (например, ML или MLOps) |

---

## 🧰 Инструменты

- [`just`](https://github.com/casey/just) — удобный task-раннер, альтернатива `make`
- `sccache` — кэш-компилятор для ускорения CI Rust-сборок
- Кэш-настройки (`RUSTC_WRAPPER`, `SCCACHE_DIR`, `CARGO_HOME`, и др.)

---

## ⚙️ Использование

### 📌 Сборка образа вручную

```bash
docker build -f devops/Dockerfile.dev-rust -t my-rust-dev .
