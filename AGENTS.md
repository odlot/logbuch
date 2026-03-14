## Principles

- **Minimal dependencies.** Do not introduce external crates without explicit approval.
- **Approved crates:** `serde` (with `derive`), `serde_json`, `chrono`, `clap` (with `derive` feature).
- **Error handling:** `std::io::Error` — no `anyhow`, no `thiserror`.
- **Ask before adding crates.** If a problem seems to require an external crate, discuss alternatives first.

## Git Workflow

- `main` — stable/release branch (protected)
- `develop` — integration branch; all feature branches branch off from here
- Feature branches: `feature/<name>` — merged into `develop` via PR
- CI runs on PRs targeting `develop`
- Merging to `develop` triggers: fast-forward push to `main` + create GitHub Release
- Semantic versioning starting at `0.1.0`, patch auto-incremented on each release workflow run

## Commits & PRs

- Use **conventional commits** (e.g. `feat:`, `fix:`, `docs:`, `ci:`, `refactor:`, `test:`, `chore:`)
- Keep commits small and focused — one logical change per commit
- Keep PRs as small as possible. If a feature is too large for a single PR, use **stacked PRs** to split it into reviewable increments
- **Never force-push.** Force-pushing outdates review comments on PRs. Always push new commits instead

## Data Model

- **Logbuch** — top-level structure, contains a list of `Log` entries
- **Log** — represents one day, contains a `timestamp` (full ISO 8601) and a list of `Note` entries
- **Note** — a single timestamped entry with `timestamp` (ISO 8601) and `description`

When adding a note, find or create the `Log` for today's date, then append the note to it.

## List Output Format

Print the date as a markdown heading, then a bullet point list of notes beneath:

```
# 2026-03-14

- 10:30 my first note
- 14:15 another note

# 2026-03-13

- 09:00 yesterday's note
```

## Storage

- File: `$XDG_DATA_HOME/logbuch/logbuch.json` (default: `~/.local/share/logbuch/logbuch.json`)
- Override: `LOGBUCH_DATA_HOME` env var (points to directory; file is always `logbuch.json`)

## Resolved Decisions

| Question | Decision | Rationale |
|----------|----------|-----------|
| Timestamp crate | `chrono` approved | `std` has no ISO 8601 formatter; hand-rolling is error-prone |
| macOS build | `macos-latest` runner (native Apple Silicon) | Cannot cross-compile for darwin from Linux |
| Linux ARM64 build | `ubuntu-24.04-arm` native runner | `cross` tool unmaintained (no release since Feb 2023, stale Docker images); native runner is simpler and faster |
| CLI parser | `clap` with derive | Full-featured, handles arg joining for quote-free input |
| Error handling | `std::io::Error` | No external crate |
| Storage path | XDG-compliant + `LOGBUCH_DATA_HOME` override | Standard on Linux/macOS |
| Storage file | `logbuch.json` | Generic name to support future entry types |
| List format | `HH:MM description` grouped by date | Clean, readable terminal output |
| develop → main | Fast-forward push | Clean linear history |
