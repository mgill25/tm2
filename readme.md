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
- `fold_ok`: Does a `fold` over an input collection of `Option` types. `fold` has an accumulator. If there is a `None` that is encountered, we can choose to just skip over it.
- `.is_ok()` can be used instead of redundant single-use `if let Ok(_)` statements.
- There are only limited things we can do when defining `const` values:
	- `calls in constants are limited to constant functions, tuple structs and tuple variants`

## Questions

- How does the `use` statement work? Does it matter if we add `use` in a method call and don't hoist it to the top?
- When should we decide b/w using `&str` and `String` types?
- `.clone()`: when to use and how to avoid.
- `pattern-matching` with variables: possible if we have an `if pattern == variable` in our arms.
- What is a `constant function` ?