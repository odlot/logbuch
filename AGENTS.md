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
- **Squash merge only.** All PRs into `develop` use squash merge to keep linear history. `main` requires linear history (no merge commits)

## Data Model

- **Logbuch** — top-level structure, contains a list of `Todo` entries
- **Todo** — a work item with `timestamp` (creation timestamp, unique identifier), `description`, `done` flag, and a list of `Session` entries
- **Session** — a pomodoro work session with `begin`, `end` (None while active), `duration` (in minutes), and a list of `Note` entries
- **Note** — a single timestamped entry with `timestamp` (ISO 8601) and `description`

Flat list of todos. No hierarchy, no grouping, no inbox. Simple and clean.

## CLI Commands

| Command | Description |
|---------|-------------|
| `logbuch add <text>` | Create a todo |
| `logbuch list` | Show all todos (undone first, then done) |
| `logbuch start <index>` | Start a pomodoro session on a todo (foreground) |
| `logbuch toggle [index]` | Toggle a todo done/undone (shows list if no index given) |

There is no `stop` command. Sessions end either by timer expiry (auto-stop) or by the user pressing Ctrl+C.

## Foreground Session Input

| Input | Behavior |
|---|---|
| `some text` | Add a **note** to the current session |
| `/todo some text` | Create a new **todo** in the flat list |

Show `>` symbol to denote input mode.

## List Output Format

```
  1. [ ] design login flow (1 session, 25 min)
  2. [ ] write middleware (0 sessions)
  3. [ ] check redis (0 sessions)

  Done:
  4. [x] setup CI (1 session, 25 min)
```

Shows undone todos first, then done todos.

## Session (Pomodoro)

- **Foreground only** — no background mode. The user focuses on one todo at a time
- Duration is **always prompted** on start, with last used value as default
- Last chosen duration becomes the new default (persisted in config)
- On new config, default is 25 minutes
- Shows countdown, `>` prompt for note/todo input
- Auto-stops on timer expiry with notification
- Ctrl+C saves session with current timestamp as end

## Configuration

- File: `logbuch.config.json` in same directory as data
- Stores `default_duration` (in minutes, default: 25, updated each time user chooses a duration)

## Storage

- File: `$XDG_DATA_HOME/logbuch/logbuch.json` (default: `~/.local/share/logbuch/logbuch.json`)
- Override: `LOGBUCH_DATA_HOME` env var (points to directory; file is always `logbuch.json`)

## Resolved Decisions

| Question | Decision | Rationale |
|----------|----------|-----------|
| Timestamp crate | `chrono` approved | `std` has no ISO 8601 formatter; hand-rolling is error-prone |
| macOS build | `macos-latest` runner (native Apple Silicon) | Cannot cross-compile for darwin from Linux |
| Linux ARM64 build | `ubuntu-24.04-arm` native runner | `cross` tool unmaintained; native runner is simpler and faster |
| CLI parser | `clap` with derive | Full-featured, handles arg joining for quote-free input |
| Error handling | `std::io::Error` | No external crate |
| Storage path | XDG-compliant + `LOGBUCH_DATA_HOME` override | Standard on Linux/macOS |
| Storage file | `logbuch.json` | Generic name |
| Config file | `logbuch.config.json` | Separate from data, same directory |
| List format | Indexed todos with session count/time | Clean, readable terminal output |
| Session mode | Foreground only | Single-todo focus, no multitasking |
| Data model | Flat todo list, no hierarchy | Minimalism — grouping happens in issue trackers, not here |
| develop → main | Fast-forward push | Clean linear history |
