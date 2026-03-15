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

- **Logbuch** — top-level structure, contains a list of `Log` entries
- **Log** — represents one day, contains a `timestamp` (full ISO 8601) and a list of `Task` entries
- **Task** — a piece of work with `timestamp` (creation timestamp, unique identifier, never changes), `description`, `done` flag, and a list of `Session` entries
- **Session** — a pomodoro work session with `begin`, `end` (None while active), `duration` (in minutes), and a list of `Note` entries
- **Note** — a single timestamped entry with `timestamp` (ISO 8601) and `description`

Notes can only be added during an active session (foreground mode). Tasks carry over between days: on each CLI invocation, unfinished tasks (done == false) from previous days are copied into today's Log (matched by timestamp, empty sessions for the new day).

## CLI Commands

| Command | Description |
|---------|-------------|
| `logbuch add <description>` | Create a new task in today's log |
| `logbuch list` | Show today's active (not done) tasks with indices |
| `logbuch start <index> [--duration <min>]` | Start a pomodoro session (foreground, accepts notes inline) |
| `logbuch toggle <index>` | Toggle a task between done and undone |

There is no `stop` command. Sessions end either by timer expiry (auto-stop) or by the user pressing Ctrl+C in the foreground session. The CLI is always in foreground during an active session — there is no background mode. The user focuses on one task at a time.

`list` is only usable when no session is active (the CLI is in foreground during sessions). To see tasks, the user must first end the current session.

## List Output Format

```
# 2026-03-14

  1. Build feature X (2 sessions, 50 min)
  2. Fix bug Y (1 session, 25 min)
  3. Write docs (0 sessions)
```

Only shows tasks where done == false.

## Session (Pomodoro)

- **Foreground only** — no background mode. The user should always focus on one task during a session
- Default duration: 25 minutes, configurable per start via `--duration`
- Last chosen duration becomes the new default (persisted in config)
- Foreground mode: shows countdown, accepts note input (type + Enter)
- Auto-stops on timer expiry with notification
- Ctrl+C saves session with current timestamp as end
- Only one session active at a time

## Configuration

- File: `logbuch.config.json` in same directory as data
- Stores `default_duration` (in minutes, default: 25, updated on each `start --duration`)

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
| Config file | `logbuch.config.json` | Separate from data, same directory |
| List format | Indexed tasks with session count/time | Clean, readable terminal output |
| Session mode | Foreground only | Single-task focus, no multitasking |
| develop → main | Fast-forward push | Clean linear history |
