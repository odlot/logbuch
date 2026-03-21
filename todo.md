# logbuch — TODO

## Milestone 1: MVP (completed)

- [x] Project setup (cargo init, deps, .gitignore)
- [x] Core CLI (add, list)
- [x] CI (PR validation)
- [x] CD (release automation with changelog)
- [x] README
- [x] Branch protection
- [x] Dependabot

## Milestone 2: Todos & Pomodoro Sessions

### 1. Data Model Refactor

Restructure from `Logbuch > Log > Note` to `Logbuch > Todo > Session > Note`.

```rust
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

struct Todo {
    timestamp: String,       // creation timestamp (unique identifier)
    description: String,
    done: bool,
    sessions: Vec<Session>,
}

struct Logbuch {
    todos: Vec<Todo>,
}
```

- [ ] Migrate data model in code

### 2. CLI Commands

| Command | Description |
|---------|-------------|
| `logbuch add <text>` | Create a todo |
| `logbuch list` | Show all todos (undone first, then done) |
| `logbuch start <index>` | Start a pomodoro session on a todo (foreground) |
| `logbuch toggle [index]` | Toggle a todo done/undone (shows list if no index given) |

- [ ] Refactor `add` to create a Todo
- [ ] Refactor `list` to show todos with index, done status, session count, total time
- [ ] Implement `start` subcommand (prompts for duration)
- [ ] Implement `toggle` subcommand (shows list if no index given)

### 3. Foreground Session (Pomodoro Timer)

`logbuch start <index>` prompts for duration, then enters foreground mode. Foreground only — no background mode. The user focuses on one todo at a time.

- [ ] Prompt for duration on start (show default from config, accept Enter for default)
- [ ] Show countdown timer (updating in terminal)
- [ ] Show `>` symbol to denote input mode
- [ ] Accept note input inline (user types + Enter to add a note to the session)
- [ ] `/todo <text>` during session creates a new todo in the flat list
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

### 5. List Output

```
  1. [ ] design login flow (1 session, 25 min)
  2. [ ] write middleware (0 sessions)
  3. [ ] check redis (0 sessions)

  Done:
  4. [x] setup CI (1 session, 25 min)
```

- [ ] Show index, done status, description, session count, total time
- [ ] Show undone todos first, then done todos

### 6. Update README

- [ ] Document new commands: `add`, `list`, `start`, `toggle`
- [ ] Document pomodoro session workflow and `/todo` capture
- [ ] Document config file

### 7. Tests

- [ ] Todo creation and listing
- [ ] Session start/stop lifecycle
- [ ] `/todo` capture during session
- [ ] Config load/save/default behavior
