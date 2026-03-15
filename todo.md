# logbuch — TODO

## Milestone 1: MVP (completed)

- [x] Project setup (cargo init, deps, .gitignore)
- [x] Core CLI (add, list)
- [x] CI (PR validation)
- [x] CD (release automation with changelog)
- [x] README
- [x] Branch protection
- [x] Dependabot

## Milestone 2: Tasks & Sessions

### 1. Data Model Refactor

Restructure from `Logbuch > Log > Note` to `Logbuch > Log > Task > Session > Note`.

```rust
struct Note {
    timestamp: String,       // full ISO 8601
    description: String,
}

struct Session {
    begin: String,           // full ISO 8601
    end: Option<String>,     // None while active
    duration_minutes: u32,   // configured pomodoro duration
    notes: Vec<Note>,
}

struct Task {
    id: String,              // creation timestamp (unique ID)
    description: String,
    done: bool,
    sessions: Vec<Session>,
}

struct Log {
    timestamp: String,       // full ISO 8601 (day)
    tasks: Vec<Task>,
}

struct Logbuch {
    logs: Vec<Log>,
}
```

- [ ] Migrate data model in code
- [ ] Task carry-over: on each CLI invocation, copy unfinished tasks (done == false) from previous days into today's Log (if not already present, matched by id)

### 2. CLI Command Changes

All commands now operate on tasks:

| Command | Description |
|---------|-------------|
| `logbuch add <description>` | Create a new task in today's log |
| `logbuch list` | Show today's active (not done) tasks with indices |
| `logbuch start <index> [--duration <min>]` | Start a pomodoro session on a task (foreground) |
| `logbuch stop` | Manually stop the active session |
| `logbuch done <index>` | Mark a task as done (hidden from list) |

- [ ] Refactor `add` to create a Task (not a Note)
- [ ] Refactor `list` to show tasks with index, description, session count
- [ ] Implement `start` subcommand
- [ ] Implement `stop` subcommand
- [ ] Implement `done` subcommand
- [ ] Error on `add <note text>` when no session active — show task list with hint to start a session

### 3. Foreground Session (Pomodoro Timer)

`logbuch start <index>` enters a foreground interactive mode:

- [ ] Show countdown timer (updating in terminal)
- [ ] Accept note input inline (user types + Enter to add a note to the session)
- [ ] Auto-stop session when timer expires (set `end` timestamp, exit foreground mode)
- [ ] Notify user on session completion (terminal bell / message)
- [ ] Handle Ctrl+C gracefully: save session with current timestamp as end
- [ ] Only one session active at a time — error if another is already running
- [ ] Default duration: 25 minutes
- [ ] Optional `--duration <minutes>` flag overrides default

### 4. Configuration

Config file: `logbuch.config.json` in same directory as data (XDG / `LOGBUCH_DATA_HOME`).

```json
{
  "default_duration_minutes": 25
}
```

- [ ] Load config on startup, create with defaults if missing
- [ ] Default duration = last chosen duration (update config each time `start` is called with a duration)
- [ ] Read default from config when `--duration` is not provided

### 5. Task Carry-Over Logic

- [ ] On any CLI invocation, check if today's Log exists
- [ ] If not, create today's Log and copy all tasks with `done == false` from the most recent previous Log
- [ ] Carried-over tasks keep their original `id` but start with an empty sessions list for the new day

### 6. List Output

```
# 2026-03-14

  1. Build feature X (2 sessions, 50 min)
  2. Fix bug Y (1 session, 25 min)
  3. Write docs (0 sessions)
```

- [ ] Show index, description, session count, total time
- [ ] Only show tasks where `done == false`
- [ ] Indicate if a session is currently active (e.g. `▶ Build feature X (in progress, 12:30 remaining)`)

### 7. Update README

- [ ] Document new commands: `add`, `list`, `start`, `stop`, `done`
- [ ] Document pomodoro session workflow
- [ ] Document config file

### 8. Tests

- [ ] Task creation and carry-over logic
- [ ] Session start/stop lifecycle
- [ ] Config load/save/default behavior
