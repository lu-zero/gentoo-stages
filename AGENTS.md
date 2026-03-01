# Project Conventions

## Build Commands

```bash
cargo test                                # Run all tests (unit + doc)
cargo clippy --all-targets -- -D warnings # Lint — must be warning-free
cargo fmt --check                         # Format check — must pass
cargo doc --no-deps                       # Build docs — must have no warnings
cargo run --example list                  # Smoke-test the list example
cargo run --example download              # Smoke-test the download example
```

## Architecture

- One primary type per module (`stage3.rs` -> `Stage3`, etc.)
- Modules are private (`mod`, not `pub mod`); public API is flat re-exports in `lib.rs`
- Uses standard library and minimal dependencies
- Parser functions are `pub(crate)`, types and their methods are `pub`

## Dependencies

Minimal: currently only standard library dependencies. Any new dependency must be
justified. Prefer standard library solutions where reasonable.

## Coding Style

- `rustfmt` — all code must be formatted
- No dead code, no unused dependencies
- Doc comments on all public types, fields, and enum variants
- Keep logic in the module alongside its type
- Tests live in a `#[cfg(test)] mod tests` block at the bottom of each module

## Commits

[Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new functionality
- `fix:` — bug fix
- `refactor:` — code restructuring without behaviour change
- `docs:` — documentation only
- `test:` — adding or updating tests
- `ci:` — CI/CD changes
- `chore:` — maintenance (dependencies, tooling)

## MSRV

Minimum Supported Rust Version is **1.71.0**. CI tests against both stable and
MSRV. Do not use features that require a newer version without updating
`rust-version` in `Cargo.toml` and the CI matrix.

## Slop Warning

This codebase was largely AI-generated. Be skeptical of existing code — it may
contain bugs, incomplete coverage, or surprising edge-case behaviour.
Do not assume existing patterns are correct; verify against the actual requirements.
