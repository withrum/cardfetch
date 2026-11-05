# cardfetch 🎴

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

Минималистичная, молниеносная CLI-утилита на Rust (альтернатива `fastfetch` / `neofetch`), которая отображает системную информацию в виде аккуратной карточки со скругленными границами.

<img width="626" height="426" alt="image" src="https://github.com/user-attachments/assets/3c350b64-1b69-4ac3-9d20-ac0f022e5db8" /> <img width="626" height="426" alt="image" src="https://github.com/user-attachments/assets/b8f03719-7338-4b7e-8495-e20088d169f2" />



## ⚡ Бенчмарки и оптимизация

Замеры производительности проводились на Linux (EndeavourOS, ядро 7.2.11-zen, 100 итераций):

| Утилита | Размер бинарника | Среднее время (mean) | Мин. время (min) | Ускорение |
| :--- | :---: | :---: | :---: | :---: |
| **`cardfetch`** | **~404 КиБ** | **0.68 мс** | **0.62 мс** | **~21x быстрее** |
| `fastfetch` | ~1.87 МиБ | 14.20 мс | 13.49 мс | baseline |

### За счет чего достигнута такая скорость и компактность:
1. **Прямой парсинг системных структур**: прямое чтение `/proc` (`uptime`, `meminfo`, `osrelease`) и системный вызов `libc::statvfs` для диска вместо медленного порождения дочерних подпроцессов шелла (`df`, `pacman`, `sh`).
2. **Link-Time Optimization (LTO)**: `lto = true` устраняет неиспользуемый код между модулями и зависимостями на этапе линковки.
3. **Единый блок кодогенерации**: `codegen-units = 1` обеспечивает максимальный инлайнинг функций компилятором LLVM.
4. **Удаление отладочных символов**: `strip = true` отсекает таблицы символов, сокращая вес исполняемого файла на ~70%.
5. **Отключение раскрутки стека при панике**: `panic = "abort"` устраняет избыточный runtime-оверхед.

## Особенности
- **Мгновенный отклик (~0.7 мс)**: субмиллисекундная скорость выполнения без задержек.
- **Цветовая палитра терминала**: используются стандартные ANSI-цвета (1–8 / 0–7), благодаря чему утилита идеально ложится в текущую цветовую тему (Catppuccin, Gruvbox, TokyoNight, Nord и др.).
- **Nerd Fonts**: аккуратные иконки для каждого пункта и автоопределение логотипа дистрибутива.
- **Минимум зависимостей**: только легковесные `libc` и `unicode-width`.
- **Бесшумный fallback**: при сбое чтения любого поля возвращается `unknown` без паники и лишнего вывода ошибок.

## Установка и сборка

Требуется [Rust](https://rustup.rs/) и шрифт с поддержкой [Nerd Fonts](https://www.nerdfonts.com/).

```bash
# Клонирование репозитория
git clone https://github.com/withrum/cardfetch.git
cd cardfetch

# Сборка оптимизированного бинарника
cargo build --release

# Либо прямая установка в систему (~/.cargo/bin)
cargo install --path .
```

## Флаги

- `-b, --border <color>` — изменить цвет рамки (`gray`, `blue`, `cyan`, `magenta`, `green`, `yellow`, `red`, `white`).
- `-t, --no-title` — скрыть заголовок `user@host`.
- `-p, --palette-all` — вывести расширенную палитру (ANSI 0–7 и 8–15).
- `-h, --help` — справка.
- `-v, --version` — версия программы.

Цвет рамки также можно настроить через переменную окружения `CARDFETCH_BORDER` (например, в `.bashrc` или `.zshrc`):
```bash
export CARDFETCH_BORDER=cyan
```

## Лицензия

MIT © 2026 withrum
