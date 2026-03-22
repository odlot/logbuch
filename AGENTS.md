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

- **Logbuch** — top-level structure, contains a flat list of `Todo` entries and a list of `Log` entries
- **Log** — represents one day, contains a `date` (ISO 8601 date) and a list of standalone `Note` entries
- **Todo** — a work item with `timestamp` (creation timestamp, unique identifier), `description`, `done` flag, and a list of `Session` entries
- **Session** — a pomodoro work session with `begin`, `end` (None while active), `duration` (in minutes), and a list of `Note` entries
- **Note** — a single timestamped entry with `timestamp` (ISO 8601) and `description`

Todos are persistent across days. Logs hold standalone notes for a given day. The `log` command merges both sources into a chronological daily or weekly view.

## REPL

`logbuch` launches the REPL. There are no subcommands — all interaction happens inside the REPL. This removes friction and fits the mental model of sitting down and focusing.

## REPL Commands

| Command | Description |
|---------|-------------|
| `add <text>` | Create a todo |
| `list` | Show all todos (undone first, then done) |
| `start <index>` | Start a pomodoro session on a todo |
| `toggle [index]` | Toggle a todo done/undone (shows list if no index given) |
| `log` | Show today's work log |
| `log <date>` | Show a specific day's log |
| `log --week` | Show current week's summary |
| `quit` | Exit the REPL |

There is no `stop` command. Sessions end either by timer expiry (auto-stop) or by the user pressing Ctrl+C.

## Context-Sensitive Input

| Context | Plain text | Commands |
|---------|-----------|----------|
| Top level (REPL) | Standalone note (added to today's log) | `add`, `list`, `start`, `toggle`, `log`, `quit` |
| Active session | Session note | `/todo <text>` creates a new todo |

Show `>` prompt to denote input mode in both contexts.

## List Output Format

```
  1. [ ] design login flow (1 session, 25 min)
  2. [ ] write middleware (0 sessions)
  3. [ ] check redis (0 sessions)

  Done:
  4. [x] setup CI (1 session, 25 min)
```

Shows undone todos first, then done todos.

## Log Output Format

Daily log — entries in chronological order, interleaving standalone notes and sessions:

```
2026-03-22:

  09:02 had a quick chat with PM about scope

  design login flow
    Session: 09:15-09:45 (30 min)
    - sketched out oauth2 flow with PKCE
    - decided against session cookies, using JWT

  10:50 deployment broke staging, rolled back
```

Weekly log — aggregated by todo, total time only, all notes flattened:

```
2026-03-18 -- 2026-03-22:

  design login flow (1h 30min)
    - sketched out oauth2 flow with PKCE
    - decided against session cookies, using JWT
    - finalized token refresh strategy

  write middleware (50min)
    - auth middleware skeleton done
    - added rate limiting
```

## Session (Pomodoro)

- **Foreground only** — no background mode. The user focuses on one todo at a time
- Duration is **always prompted** on start, with last used value as default
- Last chosen duration becomes the new default (persisted in config)
- On new config, default is 25 minutes
- Shows countdown, `>` prompt for note input
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
| Interface | REPL-only, no subcommands | Removes friction, avoids command/note ambiguity, fits focused work model |
| Error handling | `std::io::Error` | No external crate |
| Storage path | XDG-compliant + `LOGBUCH_DATA_HOME` override | Standard on Linux/macOS |
| Storage file | `logbuch.json` | Generic name |
| Config file | `logbuch.config.json` | Separate from data, same directory |
| List format | Indexed todos with session count/time | Clean, readable terminal output |
| Session mode | Foreground only | Single-todo focus, no multitasking |
| Data model | Todos + daily Logs with standalone notes | Todos persist across days; Logs capture standalone notes per day; `log` command merges both |
| develop → main | Fast-forward push | Clean linear history |
