# AGENTS.md - Agentic Coding Guidelines for kcloud

This file provides guidelines for AI agents operating in this repository.

## Project Overview

- **Project**: kcloud - A minimal Claude Code implementation in Rust
- **Edition**: Rust 2021 (minimum 1.75.0)
- **Repository**: https://github.com/master-g/kcloud

---

## Build, Lint & Test Commands

### Development Build

```bash
# Build the project
cargo build

# Build with specific profile
cargo build --release

# Run the application
cargo run -- [args]
```

### Linting & Formatting

```bash
# Format code (required before commit)
cargo fmt --all

# Check formatting without modifying
cargo fmt --all -- --check

# Run clippy lints
cargo clippy -- -W warnings

# Run all lints (fmt + clippy)
cargo fmt --all && cargo clippy -- -W warnings
```

### Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc

# Check for compilation without building
cargo check
```

### Other Commands

```bash
# Generate documentation
cargo doc --no-deps

# View dependencies
cargo tree

# Check for security vulnerabilities
cargo audit
```

---

## Code Style Guidelines

### Formatting (rustfmt)

The project uses `rustfmt` with the following settings (see `.rustfmt.toml`):

- **Hard tabs** for indentation
- **Merge derives** (`#[derive(...)]` on single line)
- **Reorder imports** alphabetically
- **Reorder modules** alphabetically
- **Field init shorthand** (`Foo { field }` instead of `Foo { field: field }`)

### Lints (clippy + rustc)

See `[workspace.lints]` in `Cargo.toml`:

**Clippy (warn by default)**:
- `doc_markdown` - Warn on undocumented items
- `manual_let_else` - Warn on manual let/else
- `match_same_arms` - Warn on duplicate match arms
- `ptr_as_ptr` - Warn on raw pointer casts
- `redundant_closure_for_method_calls` - Warn on redundant closures
- `redundant_else` - Warn on unnecessary else blocks
- `ref_as_ptr` - Warn on reference-to-pointer casts
- `semicolon_if_nothing_returned` - Warn on semicolons after single-expression blocks

**Rustc**:
- `missing_docs` - Warn on undocumented public items
- `unsafe_code` - **Deny** unsafe code
- `unsafe_op_in_unsafe_fn` - Warn on unsafe operations in unsafe functions
- `unwrap_or_default` - Warn on `.unwrap_or_default()`

---

## Naming Conventions

### General Rules

| Item | Convention | Example |
|------|------------|---------|
| Modules | `snake_case` | `cli`, `logging` |
| Structs | `PascalCase` | `Cli`, `Config` |
| Enums | `PascalCase` | `Commands`, `Error` |
| Enum Variants | `PascalCase` | `SomeVariant` |
| Functions | `snake_case` | `load_config` |
| Variables | `snake_case` | `config_path` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES` |
| Traits | `PascalCase` | `Serialize` |

### Error Types

- Error enums should end with `Error` (e.g., `ConfigError`, `ToolError`)
- Error variant messages should be lowercase: `#[error("something failed: {0}")]`

---

## Type Conventions

### Primitive Types

- Use `u8`, `i32`, `u64`, etc. over `usize`/`isize` unless pointer-sized math is needed
- Prefer explicit signed/unsigned over platform-dependent types

### Collections

- `Vec<T>` for dynamically-sized sequences
- `HashMap<K, V>` for key-value stores (requires `Hash` + `Eq`)
- `BTreeMap<K, V>` for ordered key-value stores
- `HashSet<T>` for unique value collections

### Option & Result

- Use `Option<T>` for nullable values
- Use `Result<T, E>` for fallible operations
- Prefer `?` operator over `match`/`if let` for simple propagation
- Never suppress errors with `unwrap()`, `expect()`, or `as any`

---

## Error Handling

### Error Types

The project defines custom errors in `src/error.rs` using `thiserror`:

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    ReadError(String),

    #[error("missing required config: {0}")]
    MissingField(String),
}
```

### Result Types

Use the predefined `Result<T>` alias:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

### Error Handling Rules

1. **Never use `unwrap()` or `expect()`** in production code
2. **Never use `as any`** or suppress type errors
3. **Always handle errors** - return `Result<T, E>` or log appropriately
4. **Use `thiserror`** for custom error types with `#[error(...)]` macros
5. **Use `anyhow`** for application errors that don't need specific handling
6. **Chain errors** with `#[from]` attribute for automatic conversion

---

## Import Organization

Imports should be grouped and ordered:

1. Standard library (`std::`, `core::`)
2. External crates (`tokio::`, `clap::`)
3. Local modules (`crate::`, `super::`)

Within each group, sort alphabetically.

```rust
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::cli;
use crate::config;
```

---

## Documentation

### Public API

- **All public items** should have doc comments (`///` or `//!`)
- Include examples when helpful
- Use semantic line breaks (width ~80 chars)

```rust
/// Load configuration from file and environment variables.
///
/// # Errors
///
/// Returns an error if the configuration file exists but cannot be parsed.
pub fn load() -> Result<Self, Error> {
    // ...
}
```

### Module Documentation

Use `//!` for module-level docs at the top of files:

```rust
//! Error handling module
//!
//! Provides custom error types for the application.
```

---

## Testing Guidelines

### Unit Tests

- Place tests in the same file using `#[cfg(test)]` module
- Use descriptive test names: `test_name_describes_what_it_tests`
- Use `tempfile` for tests requiring filesystem operations

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let result = do_something();
        assert!(result.is_ok());
    }
}
```

---

## Git Conventions

### Commit Messages

- Use conventional commits: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Pre-commit Checks

Run before committing:

```bash
cargo fmt --all && cargo clippy -- -W warnings
```

---

## Security Considerations

- **Deny `unsafe_code`** in production
- Validate all user inputs
- Use path security checks (enabled by default in config)
- Never hardcode credentials - use environment variables

---

## Dependencies

Key dependencies (see `Cargo.toml`):

| Category | Crate | Version |
|----------|-------|---------|
| CLI | clap | 4.5 |
| Async | tokio | 1.x |
| HTTP | reqwest | 0.12 |
| Error | thiserror | 2.0 |
| Error | anyhow | 1.0 |
| Logging | tracing | 0.1 |
| Serialization | serde | 1.0 |
