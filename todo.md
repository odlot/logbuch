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

struct Break {
    begin: String,           // full ISO 8601
    end: Option<String>,     // None while active
    duration: u32,           // configured break duration in minutes
}

struct Session {
    begin: String,           // full ISO 8601
    end: Option<String>,     // None while active
    duration: u32,           // configured pomodoro duration in minutes
    notes: Vec<Note>,
    breaks: Vec<Break>,
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
- [ ] Parse known commands (`/todo`, `/list`, `/start`, `/stop`, `/toggle`, `/break`, `/coffee`, `/continue`, `/log`, `/help`, `/quit`)
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
| `/break <duration>` | Start a break (alias: `/coffee <duration>`) |
| `/continue` | End break early, resume session |
| `/stop` | Stop the current session |
| `/log` | Show today's work log |
| `/log <date>` | Show a specific day's log |
| `/log <date> <date>` | Show work log for a date range |
| `/help` | Show available commands |
| `/quit` | Exit the REPL |

- [ ] Implement `/todo` command (create a Todo)
- [ ] Implement `/list` command (show todos with index, done status, session count, total time)
- [ ] Implement `/start` command (prompts for duration, enters session mode)
- [ ] Implement `/stop` command (save session with current timestamp as end)
- [ ] Implement `/toggle` command (shows list if no index given)
- [ ] Implement `/break` command with `/coffee` alias (start break, pause session timer)
- [ ] Implement `/continue` command (end break early, resume session timer)
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

### 5. Breaks

`/break <duration>` (alias `/coffee <duration>`) starts a break during a session. The session timer pauses and the break timer counts down.

- [ ] Pause session timer on `/break`, show "(break)" suffix on session countdown
- [ ] Show break timer counting down alongside paused session timer
- [ ] End break on: timer reaches zero, Ctrl+C, or `/continue`
- [ ] Resume session timer after break ends
- [ ] Log break in session (begin, end, duration)
- [ ] Show breaks in daily log output

### 6. Configuration

Config file: `logbuch.config.json` in same directory as data (XDG / `LOGBUCH_DATA_HOME`).

```json
{
  "default_duration": 25
}
```

- [ ] Load config on startup, create with defaults if missing
- [ ] Default duration = last chosen duration (update config each time a session starts)
- [ ] On new config, default is 25 minutes

### 7. List Output

```
- [ ] design login flow (1 session, 25 min)
- [ ] write middleware (0 sessions)
- [ ] check redis (0 sessions)
- [x] setup CI (1 session, 25 min)
```

- [ ] Show done status, description, session count, total time
- [ ] Show undone todos first, then done todos

### 8. Log Output

- Entries in chronological order, interleaving standalone notes and sessions
- Date range (`/log <date> <date>`) renders one daily log per day in the range

```
# 2026-03-22

- 09:02 had a quick chat with PM about scope
- 09:15-09:45 (30min): design login flow
  - sketched out oauth2 flow with PKCE
  - decided against session cookies, using JWT
- 09:45-09:55 (10min): break
- 10:00-10:25 (25min): write middleware
  - auth middleware skeleton done
- 10:50 deployment broke staging, rolled back
```

Todos with multiple sessions on the same day appear as separate entries, one per session, in chronological order.

- [ ] Daily log: merge standalone notes, sessions, and breaks chronologically
- [ ] Date range: render each day in the range using the daily format

### 9. Update README

- [ ] Document REPL usage and commands
- [ ] Document pomodoro session workflow and `/todo` capture
- [ ] Document standalone notes and `/log` command
- [ ] Document config file

### 10. Tests

- [ ] REPL command parsing (commands vs standalone notes)
- [ ] Todo creation and listing
- [ ] Session start/stop lifecycle
- [ ] Standalone note capture to daily log
- [ ] `/todo` capture during session
- [ ] Break start/end lifecycle (timer, Ctrl+C, `/continue`)
- [ ] Break prolongs session wall-clock time
- [ ] Log output (daily and date range, with breaks)
- [ ] Config load/save/default behavior
