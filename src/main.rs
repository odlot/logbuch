use chrono::Utc;
use clap::{Parser, Subcommand};
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

#[derive(Parser)]
#[command(name = "logbuch", about = "A simple CLI for logging timestamped notes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    Add {
        /// The note text (all words joined)
        message: Vec<String>,
    },
    /// List all notes
    List,
}

fn data_path() -> PathBuf {
    let dir = if let Ok(val) = std::env::var("LOGBUCH_DATA_HOME") {
        PathBuf::from(val)
    } else if let Ok(val) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(val).join("logbuch")
    } else {
        dirs_home().join(".local/share/logbuch")
    };
    dir.join("logbuch.json")
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
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

fn add_note(message: Vec<String>) -> io::Result<()> {
    let path = data_path();
    let mut logbuch = load_logbuch(&path)?;

    let now = Utc::now();
    let today = now.format("%Y-%m-%dT00:00:00+00:00").to_string();
    let description = message.join(" ");

    let log = logbuch.logs.iter_mut().find(|l| l.timestamp == today);

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
    let path = data_path();
    let logbuch = load_logbuch(&path)?;

    if logbuch.logs.is_empty() {
        println!("No notes yet.");
        return Ok(());
    }

    for log in &logbuch.logs {
        let date = &log.timestamp[..10];
        println!("# {}", date);
        println!();
        for note in &log.notes {
            let time = &note.timestamp[11..16];
            println!("- {} {}", time, note.description);
        }
        println!();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { message } => add_note(message),
        Commands::List => list_notes(),
    }
}
