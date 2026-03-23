use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
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

fn print_help() {
    println!("Commands:");
    println!("  /todo <text>        Create a todo");
    println!("  /list               Show all todos");
    println!("  /start <index>      Start a pomodoro session");
    println!("  /stop               Stop the current session");
    println!("  /toggle [index]     Toggle todo done/undone");
    println!("  /break <duration>   Start a break (alias: /coffee)");
    println!("  /continue           End break, resume session");
    println!("  /log [date] [date]  Show work log");
    println!("  /help               Show this help");
    println!("  /quit               Exit");
    println!();
    println!("Text without / prefix is added as a note.");
}

fn main() -> io::Result<()> {
    let path = data_path()?;
    let _logbuch = load_logbuch(&path)?;

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(command) = trimmed.strip_prefix('/') {
            let parts: Vec<&str> = command.splitn(2, ' ').collect();
            let cmd = parts[0];

            match cmd {
                "help" => print_help(),
                "quit" | "q" => break,
                _ => println!("Unknown command: /{cmd}"),
            }
        }
    }

    Ok(())
}
