use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct Note {
    timestamp: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Break {
    begin: String,
    end: Option<String>,
    duration: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Session {
    begin: String,
    end: Option<String>,
    duration: u32,
    notes: Vec<Note>,
    breaks: Vec<Break>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    timestamp: String,
    description: String,
    done: bool,
    sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Log {
    date: String,
    notes: Vec<Note>,
}

#[derive(Serialize, Deserialize, Default)]
struct Logbuch {
    todos: Vec<Todo>,
    logs: Vec<Log>,
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

fn main() -> io::Result<()> {
    let path = data_path()?;
    let _logbuch = load_logbuch(&path)?;
    Ok(())
}
