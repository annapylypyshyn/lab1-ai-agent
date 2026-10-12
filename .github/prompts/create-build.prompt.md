# Command: create-build

**Передумови:** Наявний базовий проєкт Rust (Cargo.toml та src/main.rs).
**Дії:** Викликає skill `build-and-test`, створює конфігурацію складання і тести[cite: 7].
**Очікувані файли:** Оновлений `src/main.rs` (або `src/lib.rs`) з тестом `BasicAddition`[cite: 8].
**Команди перевірки:** `cargo build`, `cargo test`[cite: 5].
**Повідомлення про помилку:** "Помилка складання проєкту або юніт-тест BasicAddition не пройдено."