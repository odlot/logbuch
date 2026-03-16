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

- **Logbuch** — top-level structure, contains a list of `Log` entries and an `inbox` (list of `Todo` items)
- **Log** — represents one day, contains a `timestamp` (full ISO 8601) and a list of `Task` entries
- **Task** — a piece of work with `timestamp` (creation timestamp, unique identifier, never changes), `description`, `done` flag, a list of `Todo` sub-items, and a list of `Session` entries
- **Todo** — a sub-item with `timestamp`, `description`, `done` flag. Lives either inside a Task (planned sub-task) or in the Logbuch inbox (unplanned, captured during sessions)
- **Session** — a pomodoro work session with `begin`, `end` (None while active), `duration` (in minutes), and a list of `Note` entries
- **Note** — a single timestamped entry with `timestamp` (ISO 8601) and `description`

Notes can only be added during an active session (foreground mode). Tasks carry over between days: on each CLI invocation, unfinished tasks (done == false) from previous days are copied into today's Log (matched by timestamp, empty sessions for the new day).

## Todos & Inbox (GTD / Pomodoro Philosophy)

Two contexts for todos:

- **Planning mode** (outside session): todos are sub-items of tasks. Use `logbuch todo add <task-index> <text>` to break down a task into actionable steps.
- **Focus mode** (during session): all todos go to the **inbox**. The user is in a pomodoro — every intrusive thought or unplanned item is deferred. Type `/todo some text` to capture to inbox without breaking focus.

After a session, the user triages the inbox: promote items to tasks, attach to existing tasks as todos, or mark done.

## CLI Commands

| Command | Description |
|---------|-------------|
| `logbuch add <description>` | Create a new task in today's log |
| `logbuch list` | Show today's tasks with indices (undone first, then done from same day) |
| `logbuch start <index>` | Start a pomodoro session (prompts for duration, foreground) |
| `logbuch toggle [index]` | Toggle a task done/undone (shows list if no index given) |
| `logbuch todo add <task-index> <text>` | Add a todo to a task |
| `logbuch todo list <task-index>` | Show todos for a task |
| `logbuch todo toggle <task-index> <todo-index>` | Toggle a todo done/undone |
| `logbuch inbox` | Show inbox items |
| `logbuch inbox promote <index>` | Move inbox item to a task as todo, or create new task |

There is no `stop` command. Sessions end either by timer expiry (auto-stop) or by the user pressing Ctrl+C in the foreground session.

## Foreground Session Input

| Input | Behavior |
|---|---|
| `some text` | Add a **note** to the current session |
| `/todo some text` | Capture to **inbox** (deferred, always — no exceptions during focus) |

Show `>` symbol to denote input mode.

## List Output Format

```
# 2026-03-14

  1. Build feature X (2 sessions, 50 min)
  2. Fix bug Y (1 session, 25 min)
  3. Write docs (0 sessions)

  Done:
  4. Setup CI (1 session, 25 min)
```

Shows undone tasks first, then done tasks from the same day.

## Session (Pomodoro)

- **Foreground only** — no background mode. The user should always focus on one task during a session
- Duration is **always prompted** on start, with last used value as default (no `--duration` flag)
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
| Linux ARM64 build | `ubuntu-24.04-arm` native runner | `cross` tool unmaintained (no release since Feb 2023, stale Docker images); native runner is simpler and faster |
| CLI parser | `clap` with derive | Full-featured, handles arg joining for quote-free input |
| Error handling | `std::io::Error` | No external crate |
| Storage path | XDG-compliant + `LOGBUCH_DATA_HOME` override | Standard on Linux/macOS |
| Storage file | `logbuch.json` | Generic name to support future entry types |
| Config file | `logbuch.config.json` | Separate from data, same directory |
| List format | Indexed tasks with session count/time | Clean, readable terminal output |
| Session mode | Foreground only | Single-task focus, no multitasking |
| Todos during session | Always go to inbox | GTD/Pomodoro: defer unplanned items, stay focused |
| develop → main | Fast-forward push | Clean linear history |
