# logbuch — TODO

## Overview

Terminal CLI tool for logging timestamped notes, stored as JSON. Written in Rust.

## Tasks

### 1. Project Setup
- [ ] Create `develop` branch from `main`
- [ ] `cargo init` — set up Rust project (binary crate)
- [ ] Add `.gitignore` for Rust (`/target`, etc.)
- [ ] Add `Cargo.toml` metadata (name: `logbuch`, version: `0.1.0`, edition 2024)
- [ ] Dependencies: `serde` (with `derive` feature), `serde_json`, `chrono`, `clap` (with `derive` feature)

### 2. Core CLI — MVP
- [ ] CLI argument parsing with `clap` (derive API)
- [ ] Subcommands:
  - `add <message>` — all args after `add` joined into one note (no quotes needed: `logbuch add my note`)
  - `list` — date as markdown heading, notes as bullet list beneath (`- HH:MM description`)
- [ ] Data structures:
  ```rust
  struct Note {
      timestamp: String,  // full ISO 8601 (e.g. 2026-03-14T10:30:00+00:00)
      description: String,
  }
  struct Log {
      timestamp: String,  // full ISO 8601 (e.g. 2026-03-14T00:00:00+00:00)
      notes: Vec<Note>,
  }
  struct Logbuch {
      logs: Vec<Log>,
  }
  ```
- [ ] `add`: find or create today's `Log`, append `Note` with full ISO 8601 timestamp
- [ ] `list`: print date as `# YYYY-MM-DD` heading, notes as `- HH:MM description` bullet list
- [ ] JSON storage: auto-create directory and file if missing

### 3. CI — PR Validation (`.github/workflows/ci.yml`)
- [ ] Trigger: pull requests targeting `develop`
- [ ] Steps: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- [ ] Runner: `ubuntu-latest`

### 4. CD — Release Automation (`.github/workflows/release.yml`)
- [ ] Trigger: push to `develop` (i.e. after PR merge)
- [ ] Matrix strategy with two jobs:
  | Target                        | Runner              | Toolchain setup                                              |
  |-------------------------------|---------------------|--------------------------------------------------------------|
  | `aarch64-unknown-linux-gnu`   | `ubuntu-24.04-arm`  | Native ARM64 runner — no cross-compilation needed             |
  | `aarch64-apple-darwin`        | `macos-latest`      | Native Apple Silicon runner — no cross-compilation needed     |
- [ ] Steps:
  1. Determine next version (read latest `v*` tag, increment patch; default `0.1.0` if no tag)
  2. Bump version in `Cargo.toml`, commit to `develop`
  3. Build release binary on each matrix runner (native, no cross-compilation)
  4. Fast-forward push `develop` to `main`
  5. Create git tag `v0.x.y` on `main`
  6. Create GitHub Release with tag `v0.x.y`
  7. Attach both binaries as release assets (`logbuch-linux-aarch64`, `logbuch-darwin-aarch64`)
- [ ] Starting version: `0.1.0`

### 5. README.md
- [ ] Project description
- [ ] Installation (from GitHub Releases)
- [ ] Usage: `logbuch add my note`, `logbuch list`
- [ ] Configuration: storage path (`XDG_DATA_HOME`, `LOGBUCH_DATA_HOME` override)

### 6. Branch Protection (manual, after first PR)
- [ ] Protect `main`: no direct pushes (except CI/GitHub Actions)
- [ ] Protect `develop`: require PR + CI pass
