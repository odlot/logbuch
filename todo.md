# logbuch — TODO

## Overview

Terminal CLI tool for logging timestamped notes, stored as JSON. Written in Rust.

## Git Workflow

- `main` — stable/release branch (protected)
- `develop` — integration branch; all feature branches branch off from here
- Feature branches: `feature/<name>` — merged into `develop` via PR
- CI runs on PRs targeting `develop`
- Merging to `develop` triggers: fast-forward push to `main` + create GitHub Release
- Semantic versioning starting at `0.1.0`, patch auto-incremented on each release workflow run

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
  - `list` — display all notes in markdown format
- [ ] `Note` struct:
  ```rust
  #[derive(serde::Serialize, serde::Deserialize)]
  struct Note {
      timestamp: String,  // ISO 8601 via chrono::Utc::now()
      description: String,
  }
  ```
- [ ] Timestamp: `chrono::Utc::now().to_rfc3339()` for ISO 8601
- [ ] `list` output: markdown representation (e.g. `- **2026-03-14T10:00:00Z** — my note`)
- [ ] JSON storage:
  - File: `$XDG_DATA_HOME/logbuch/logbuch.json` (default: `~/.local/share/logbuch/logbuch.json`)
  - Override: `LOGBUCH_DATA_HOME` env var (points to directory; file is always `logbuch.json`)
  - Auto-create directory and file if missing
- [ ] Document storage path and env var override in `README.md`

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
- [ ] Note: native ARM64 runners avoid `cross` tool (unmaintained since 2023, Docker overhead) and manual cross-toolchain setup entirely

### 5. README.md
- [ ] Project description
- [ ] Installation (from GitHub Releases)
- [ ] Usage: `logbuch add my note`, `logbuch list`
- [ ] Configuration: storage path (`XDG_DATA_HOME`, `LOGBUCH_DATA_HOME` override)

### 6. Branch Protection (manual, after first PR)
- [ ] Protect `main`: no direct pushes (except CI/GitHub Actions)
- [ ] Protect `develop`: require PR + CI pass

---

## Resolved Decisions

| Question | Decision | Rationale |
|----------|----------|-----------|
| Timestamp crate | `chrono` approved | `std` has no ISO 8601 formatter; hand-rolling is error-prone |
| macOS build | `macos-latest` runner (native Apple Silicon) | Cannot cross-compile for darwin from Linux |
| Linux ARM64 build | `ubuntu-24.04-arm` native runner | `cross` tool unmaintained (no release since Feb 2023, stale Docker images); manual toolchain works but native runner is simpler and faster |
| CLI parser | `clap` with derive | Full-featured, handles arg joining for quote-free input |
| Error handling | `std::io::Error` | No external crate |
| Storage path | XDG-compliant + `LOGBUCH_DATA_HOME` override | Standard on Linux/macOS |
| Storage file | `logbuch.json` | Generic name to support future entry types |
| List format | Markdown | Clean, readable terminal output |
| develop → main | Fast-forward push | Clean linear history |

## Open Questions

None — all questions resolved. Ready for implementation.
