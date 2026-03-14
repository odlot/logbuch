# logbuch

A simple CLI for logging timestamped notes, stored as JSON.

## Installation

Download the latest binary from [GitHub Releases](https://github.com/odlot/logbuch/releases):

- `logbuch-linux-aarch64` — Linux (ARM64)
- `logbuch-darwin-aarch64` — macOS (Apple Silicon)

```sh
chmod +x logbuch-*
mv logbuch-* /usr/local/bin/logbuch
```

## Usage

### Add a note

```sh
logbuch add my first note
```

No quotes needed — all arguments after `add` are joined into a single note.

### List notes

```sh
logbuch list
```

Output is grouped by date:

```
# 2026-03-14

- 10:30 my first note
- 14:15 another note

# 2026-03-13

- 09:00 yesterday's note
```

## Configuration

Notes are stored as JSON at:

```
$XDG_DATA_HOME/logbuch/logbuch.json
```

If `XDG_DATA_HOME` is not set, you must set `LOGBUCH_DATA_HOME` to specify the storage directory:

```sh
export LOGBUCH_DATA_HOME=~/.logbuch
```

The file is always named `logbuch.json` within the configured directory. The directory and file are created automatically on first use.
