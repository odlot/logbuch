# logbuch — TODO

## Milestone 1: MVP (completed)

- [x] Project setup (cargo init, deps, .gitignore)
- [x] Core CLI (add, list)
- [x] CI (PR validation)
- [x] CD (release automation with changelog)
- [x] README
- [x] Branch protection
- [x] Dependabot

## Milestone 2: REPL, Todos & Pomodoro Sessions

### 1. Data Model Refactor

Restructure from `Logbuch > Log > Note` to `Logbuch > { Todo, Log }`.

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

struct Log {
    date: String,            // ISO 8601 date (e.g. "2026-03-22")
    notes: Vec<Note>,        // standalone notes (quick dumps)
}

struct Logbuch {
    todos: Vec<Todo>,        // flat, persistent across days
    logs: Vec<Log>,          // daily logs for standalone notes
}
```

- [ ] Migrate data model in code

### 2. REPL

`logbuch` launches the REPL. No subcommands — all interaction happens inside the REPL.

- [ ] Remove clap, implement REPL loop (read line, parse command, execute)
- [ ] `/` prefix = command, no prefix = note (same rule in both contexts)
- [ ] Parse known commands (`/todo`, `/list`, `/start`, `/stop`, `/toggle`, `/log`, `/help`, `/quit`)
- [ ] Treat unprefixed input as standalone note (add to today's log)
- [ ] Handle empty input gracefully (no-op)

### 3. REPL Commands

All commands are prefixed with `/`. Input without `/` is always a note.

| Command | Description |
|---------|-------------|
| `/todo <text>` | Create a todo |
| `/list` | Show all todos (undone first, then done) |
| `/start <index>` | Start a pomodoro session on a todo |
| `/toggle [index]` | Toggle a todo done/undone (shows list if no index given) |
| `/log` | Show today's work log |
| `/log <date>` | Show a specific day's log |
| `/log <date> <date>` | Show work log for a date range |
| `/stop` | Stop the current session |
| `/help` | Show available commands |
| `/quit` | Exit the REPL |

- [ ] Implement `/todo` command (create a Todo)
- [ ] Implement `/list` command (show todos with index, done status, session count, total time)
- [ ] Implement `/start` command (prompts for duration, enters session mode)
- [ ] Implement `/stop` command (save session with current timestamp as end)
- [ ] Implement `/toggle` command (shows list if no index given)
- [ ] Implement `/log` command (today, specific date, date range)
- [ ] Implement `/help` command (list available commands)
- [ ] Implement `/quit` command

### 4. Foreground Session (Pomodoro Timer)

`/start <index>` prompts for duration, then enters foreground mode. Foreground only — no background mode. The user focuses on one todo at a time.

- [ ] Prompt for duration on start (show default from config, accept Enter for default)
- [ ] Show countdown timer (updating in terminal)
- [ ] Show `>` prompt to denote input mode
- [ ] Accept note input inline (user types + Enter to add a note to the session)
- [ ] All `/` commands available during session (e.g. `/todo` to create a new todo)
- [ ] Auto-stop session when timer expires (set `end` timestamp, return to REPL)
- [ ] Notify user on session completion (terminal bell / message)
- [ ] Handle Ctrl+C gracefully: save session with current timestamp as end
- [ ] Update config with last chosen duration as new default

### 5. Configuration

Config file: `logbuch.config.json` in same directory as data (XDG / `LOGBUCH_DATA_HOME`).

```json
{
  "default_duration": 25
}
```

- [ ] Load config on startup, create with defaults if missing
- [ ] Default duration = last chosen duration (update config each time a session starts)
- [ ] On new config, default is 25 minutes

### 6. List Output

```
  1. [ ] design login flow (1 session, 25 min)
  2. [ ] write middleware (0 sessions)
  3. [ ] check redis (0 sessions)

  Done:
  4. [x] setup CI (1 session, 25 min)
```

- [ ] Show index, done status, description, session count, total time
- [ ] Show undone todos first, then done todos

### 7. Log Output

- Entries in chronological order, interleaving standalone notes and sessions
- Date range (`/log <date> <date>`) renders one daily log per day in the range

```
2026-03-22:

  09:02 had a quick chat with PM about scope

  design login flow
    Session: 09:15-09:45 (30 min)
    - sketched out oauth2 flow with PKCE
    - decided against session cookies, using JWT

  10:50 deployment broke staging, rolled back
```

- [ ] Daily log: merge standalone notes and sessions chronologically
- [ ] Date range: render each day in the range using the daily format

### 8. Update README

- [ ] Document REPL usage and commands
- [ ] Document pomodoro session workflow and `/todo` capture
- [ ] Document standalone notes and `/log` command
- [ ] Document config file

### 9. Tests

- [ ] REPL command parsing (commands vs standalone notes)
- [ ] Todo creation and listing
- [ ] Session start/stop lifecycle
- [ ] Standalone note capture to daily log
- [ ] `/todo` capture during session
- [ ] Log output (daily and date range)
- [ ] Config load/save/default behavior
