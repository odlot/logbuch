use chrono::{DateTime, NaiveDate, Utc};
use clap::{Command, arg};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Note {
    timestamp: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Log {
    timestamp: String,
    notes: Vec<Note>,
}

#[derive(Serialize, Deserialize, Default)]
struct Logbuch {
    logs: Vec<Log>,
}

fn cli() -> Command {
    Command::new("logbuch")
        .about("A simple CLI for logging timestamped notes")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a new note")
                .arg(arg!(<MESSAGE> ... "The note text")),
        )
        .subcommand(Command::new("list").about("List all notes"))
}

fn data_path() -> io::Result<PathBuf> {
    let dir = if let Ok(val) = std::env::var("LOGBUCH_DATA_HOME") {
        PathBuf::from(val)
    } else if let Ok(val) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(val).join("logbuch")
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Neither LOGBUCH_DATA_HOME nor XDG_DATA_HOME is set",
        ));
    };
    Ok(dir.join("logbuch.json"))
}

fn load_logbuch(path: &PathBuf) -> io::Result<Logbuch> {
    if path.exists() {
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(Logbuch::default())
    }
}

fn save_logbuch(path: &PathBuf, logbuch: &Logbuch) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(logbuch)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, data)
}

fn parse_date(timestamp: &str) -> Option<NaiveDate> {
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.date_naive())
}

fn format_time(timestamp: &str) -> String {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|dt| dt.format("%H:%M").to_string())
        .unwrap_or_else(|_| "??:??".to_string())
}

fn add_note(message: Vec<String>) -> io::Result<()> {
    let path = data_path()?;
    let mut logbuch = load_logbuch(&path)?;

    let now = Utc::now();
    let today = now.to_rfc3339();
    let description = message.join(" ");

    let today_date = now.date_naive();
    let log = logbuch
        .logs
        .iter_mut()
        .find(|l| parse_date(&l.timestamp) == Some(today_date));

    let note = Note {
        timestamp: now.to_rfc3339(),
        description,
    };

    match log {
        Some(log) => log.notes.push(note),
        None => logbuch.logs.push(Log {
            timestamp: today,
            notes: vec![note],
        }),
    }

    save_logbuch(&path, &logbuch)?;
    Ok(())
}

fn list_notes() -> io::Result<()> {
    let path = data_path()?;
    let logbuch = load_logbuch(&path)?;

    if logbuch.logs.is_empty() {
        println!("No notes yet.");
        return Ok(());
    }

    for log in &logbuch.logs {
        let date = parse_date(&log.timestamp)
            .map(|d| d.to_string())
            .unwrap_or_else(|| "Unknown date".to_string());
        println!("# {}", date);
        println!();
        for note in &log.notes {
            println!("- {} {}", format_time(&note.timestamp), note.description);
        }
        println!();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub)) => {
            let message: Vec<String> = sub
                .get_many::<String>("MESSAGE")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            add_note(message)
        }
        Some(("list", _)) => list_notes(),
        _ => unreachable!(),
    }
}
