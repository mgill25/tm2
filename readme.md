# TM - The alacritty Theme Changer

I change themes frequently during my Terminal sessions in alacritty.
For base16 themes, there is also a lot of overlap between themes used by the terminal as well as themes used by vim.

This tool is just a small little wrapper that will allow theme changes.

## Rust learnings

- Pattern matching (need to learn more _patterns_ of Pattern Matching!)
- `log` und `env_logger`
- `std::fs::read_dir`
- Splitting up a chain of `.unwrap()` into let bindings to resolve borrow-checker errors.
- `const` types need types to be provided.
- `.collect<T>()` where `T` can be `Vec<&str>` or other types. AKA `.collect<Vec<&str>>()`.
- `s("foo")` convenience fn which internally does `.to_string()` (reduces noise)
- Single match can be just a `let` statement, as per `Clippy` (variable pattern matching like Python)
- `&String` can usually be converted to `&str`, and Clippy will remind you of this. So use him!

