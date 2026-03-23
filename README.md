# logbuch

A pomodoro-style work logger with todos, sessions, breaks, and daily logs.

## Installation

Download the latest binary from [GitHub Releases](https://github.com/odlot/logbuch/releases):

- `logbuch-linux-aarch64` — Linux (ARM64)
- `logbuch-darwin-aarch64` — macOS (Apple Silicon)

```sh
chmod +x logbuch-*
mv logbuch-* /usr/local/bin/logbuch
```

## Usage

Run `logbuch` to launch the interactive REPL. All interaction happens inside the REPL — there are no subcommands.

```
$ logbuch
> /todo design login flow
> /todo write middleware
> /list
- [ ] design login flow (0 sessions)
- [ ] write middleware (0 sessions)
> /start 1
Duration [25]: 30
  30:00 -- design login flow > sketched out oauth2 flow with PKCE
  24:12 -- design login flow > decided against session cookies
  00:00 -- design login flow >
Session complete! (30 min)
> /log
# 2026-03-22

- 09:15-09:45 (30min): design login flow
  - sketched out oauth2 flow with PKCE
  - decided against session cookies
> /quit
```

### Commands

All commands are prefixed with `/`. Text without `/` is added as a standalone note to today's log.

| Command | Description |
|---------|-------------|
| `/todo <text>` | Create a todo |
| `/list` | Show all todos (undone first, then done) |
| `/start <index>` | Start a pomodoro session on a todo |
| `/stop` | Stop the current session |
| `/toggle [index]` | Toggle a todo done/undone |
| `/break <duration>` | Start a break (alias: `/coffee`) |
| `/continue` | End break early, resume session |
| `/log` | Show today's work log |
| `/log <date>` | Show a specific day's log |
| `/log <date> <date>` | Show work log for a date range |
| `/help` | Show available commands |
| `/quit` | Exit the REPL |

### Sessions

`/start <index>` prompts for a duration (default from config) and enters foreground mode. During a session:

- Type text to add a note to the session
- `/todo <text>` captures a new todo without leaving the session
- `/break <minutes>` pauses the session timer for a break
- `/stop` or timer expiry ends the session

### Standalone notes

Text entered without a `/` prefix is saved as a standalone note in today's daily log. These appear alongside session entries in `/log` output.

## Configuration

### Data storage

Data is stored as JSON at:

```
$XDG_DATA_HOME/logbuch/logbuch.json
```

Override with `LOGBUCH_DATA_HOME`:

```sh
export LOGBUCH_DATA_HOME=~/.logbuch
```

### Config file

Session defaults are stored in `logbuch.config.json` (same directory as data):

```json
{
  "default_duration": 25
}
```

The default duration updates automatically to the last chosen value.
