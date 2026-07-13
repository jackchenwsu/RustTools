# Repository Guidelines

## Project Structure & Module Organization

This is a Rustlings-style practice workspace. Work in `exercises/<topic>/`,
where numbered folders follow the learning sequence (for example,
`exercises/21_macros/macros2.rs`). Each topic has a short `README.md`.
`solutions/` mirrors the exercise paths and is reserved for reference or
generated solutions. `Cargo.toml` registers every exercise and solution as an
explicit binary; add a matching entry when adding a new exercise.

## Build, Test, and Development Commands

- `cargo build --bin macros2` builds one exercise. Use this as the normal
  validation command while solving it.
- `cargo run --bin intro1` builds and runs an executable exercise.
- `cargo test --bin tests1` runs tests embedded in a single exercise.
- `cargo fmt --check` checks formatting; use `cargo fmt` to apply it.
- `cargo clippy --bin clippy1 -- -D warnings` checks a completed exercise.

Do not expect `cargo build --bins` or `cargo test --all` to pass while other
learning exercises are intentionally incomplete. To validate all reference
solutions, run:

```bash
awk -F'"' '/name = ".*_sol"/ { print "--bin", $2 }' Cargo.toml | xargs cargo build
```

## Coding Style & Naming Conventions

Use standard Rust formatting with four-space indentation; run `cargo fmt`
before sharing changes. Keep exercise filenames lowercase and numbered (for
example, `strings3.rs`), and preserve the parallel path under `solutions/`.
Prefer idiomatic `snake_case` for functions and variables and `PascalCase` for
types. Keep each exercise focused on its stated concept; do not add helper
modules or dependencies unless the exercise requires them.

## Testing Guidelines

Place unit tests in the exercise file using `#[cfg(test)]` and `#[test]`.
Name tests for the behavior they verify, such as `converts_celsius_to_fahrenheit`.
Run the affected binary's tests after every change. The manifest forbids unsafe
code and unstable features and rejects `todo!()` through Clippy.

## Commit & Pull Request Guidelines

This checkout has no commit history, so no established commit convention is
available. Use concise imperative subjects scoped to the exercise, such as
`macros: complete macros2`. Keep each commit limited to one exercise. Pull
requests should state the exercise changed, the validation command and result,
and any intentional deviation from the exercise instructions.
