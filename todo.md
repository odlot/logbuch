# logbuch — TODO

## Milestone 1: MVP (completed)

- [x] Project setup (cargo init, deps, .gitignore)
- [x] Core CLI (add, list)
- [x] CI (PR validation)
- [x] CD (release automation with changelog)
- [x] README
- [x] Branch protection
- [x] Dependabot

## Milestone 2: Tasks, Sessions & Todos

### 1. Data Model Refactor

Restructure from `Logbuch > Log > Note` to full hierarchy with inbox.

```rust
struct Todo {
    timestamp: String,       // creation timestamp (unique identifier)
    description: String,
    done: bool,
}

struct Note {
    timestamp: String,       // full ISO 8601
    description: String,
}

struct Session {
    begin: String,           // full ISO 8601
    end: Option<String>,     // None while active
    duration: u32,           // configured pomodoro duration in minutes
    notes: Vec<Note>,
}

struct Task {
    timestamp: String,       // creation timestamp (unique identifier, never changes)
    description: String,
    done: bool,
    todos: Vec<Todo>,        // planned sub-items
    sessions: Vec<Session>,
}

struct Log {
    timestamp: String,       // full ISO 8601 (day)
    tasks: Vec<Task>,
}

struct Logbuch {
    logs: Vec<Log>,
    inbox: Vec<Todo>,        // unplanned items captured during sessions
}
```

- [ ] Migrate data model in code
- [ ] Task carry-over: on each CLI invocation, copy unfinished tasks (done == false) from previous days into today's Log (if not already present, matched by timestamp)

### 2. CLI Command Changes

| Command | Description |
|---------|-------------|
| `logbuch add <description>` | Create a new task in today's log |
| `logbuch list` | Show today's tasks (undone first, then done from same day) |
| `logbuch start <index>` | Start a pomodoro session (prompts for duration, foreground) |
| `logbuch toggle [index]` | Toggle a task done/undone (shows list if no index given) |
| `logbuch todo add <task-index> <text>` | Add a todo to a task (planning mode) |
| `logbuch todo list <task-index>` | Show todos for a task |
| `logbuch todo toggle <task-index> <todo-index>` | Toggle a todo done/undone |
| `logbuch inbox` | Show inbox items |
| `logbuch inbox promote <index>` | Move inbox item to a task as todo, or create new task |

- [ ] Refactor `add` to create a Task (not a Note)
- [ ] Refactor `list` to show tasks with index, description, session count (undone first, then done from same day)
- [ ] Implement `start` subcommand (prompts for duration)
- [ ] Implement `toggle` subcommand (shows list if no index given)
- [ ] Implement `todo add`, `todo list`, `todo toggle` subcommands
- [ ] Implement `inbox`, `inbox promote` subcommands

### 3. Foreground Session (Pomodoro Timer)

`logbuch start <index>` prompts for duration (default from config), then enters foreground mode. This is the only mode — no background mode. The user focuses on one task at a time.

- [ ] Prompt for duration on start (show default from config, accept Enter for default)
- [ ] Show countdown timer (updating in terminal)
- [ ] Show `>` symbol to denote input mode
- [ ] Accept note input inline (user types + Enter to add a note to the session)
- [ ] `/todo <text>` during session captures to inbox (deferred, GTD/Pomodoro philosophy)
- [ ] Auto-stop session when timer expires (set `end` timestamp, exit foreground mode)
- [ ] Notify user on session completion (terminal bell / message)
- [ ] Handle Ctrl+C gracefully: save session with current timestamp as end
- [ ] Update config with last chosen duration as new default

### 4. Configuration

Config file: `logbuch.config.json` in same directory as data (XDG / `LOGBUCH_DATA_HOME`).

```json
{
  "default_duration": 25
}
```

- [ ] Load config on startup, create with defaults if missing
- [ ] Default duration = last chosen duration (update config each time a session starts)
- [ ] On new config, default is 25 minutes

### 5. Task Carry-Over Logic

- [ ] On any CLI invocation, check if today's Log exists
- [ ] If not, create today's Log and copy all tasks with `done == false` from the most recent previous Log
- [ ] Carried-over tasks keep their original `timestamp` but start with an empty sessions list for the new day

### 6. List Output

```
# 2026-03-14

  1. Build feature X (2 sessions, 50 min)
  2. Fix bug Y (1 session, 25 min)
  3. Write docs (0 sessions)

  Done:
  4. Setup CI (1 session, 25 min)
```

- [ ] Show index, description, session count, total time
- [ ] Show undone tasks first, then done tasks from the same day

### 7. Update README

- [ ] Document new commands: `add`, `list`, `start`, `toggle`, `todo`, `inbox`
- [ ] Document pomodoro session workflow and `/todo` capture
- [ ] Document config file

### 8. Tests

- [ ] Task creation and carry-over logic
- [ ] Session start/stop lifecycle
- [ ] Todo add/toggle/promote
- [ ] Inbox capture during session
- [ ] Config load/save/default behavior
