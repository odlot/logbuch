use chrono::Utc;
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

fn save_logbuch(path: &PathBuf, logbuch: &Logbuch) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(logbuch)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, data)
}

fn display_order(logbuch: &Logbuch) -> Vec<usize> {
    let mut order: Vec<usize> = Vec::new();
    for (i, todo) in logbuch.todos.iter().enumerate() {
        if !todo.done {
            order.push(i);
        }
    }
    for (i, todo) in logbuch.todos.iter().enumerate() {
        if todo.done {
            order.push(i);
        }
    }
    order
}

fn session_stats(todo: &Todo) -> (usize, u32) {
    let count = todo.sessions.len();
    let mins: u32 = todo.sessions.iter().map(|s| s.duration).sum();
    (count, mins)
}

fn list_todos(logbuch: &Logbuch) {
    if logbuch.todos.is_empty() {
        println!("No todos.");
        return;
    }
    let order = display_order(logbuch);
    for &idx in &order {
        let todo = &logbuch.todos[idx];
        let mark = if todo.done { "x" } else { " " };
        let (count, mins) = session_stats(todo);
        if count > 0 {
            let s = if count == 1 { "" } else { "s" };
            println!(
                "- [{mark}] {} ({count} session{s}, {mins} min)",
                todo.description
            );
        } else {
            println!("- [{mark}] {} (0 sessions)", todo.description);
        }
    }
}

fn resolve_display_index(logbuch: &Logbuch, args: &str) -> Option<usize> {
    let display_idx: usize = args.parse().ok()?;
    let order = display_order(logbuch);
    if display_idx == 0 || display_idx > order.len() {
        None
    } else {
        Some(order[display_idx - 1])
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    default_duration: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_duration: 25,
        }
    }
}

fn config_path() -> io::Result<PathBuf> {
    let data = data_path()?;
    let dir = data.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Cannot determine config directory")
    })?;
    Ok(dir.join("logbuch.config.json"))
}

fn load_config(path: &PathBuf) -> io::Result<Config> {
    if path.exists() {
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(Config::default())
    }
}

fn save_config(path: &PathBuf, config: &Config) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, data)
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
    let config_path = config_path()?;
    let mut logbuch = load_logbuch(&path)?;
    let mut _config = load_config(&config_path)?;
    save_config(&config_path, &_config)?;

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
            let args = parts.get(1).copied().unwrap_or("");

            match cmd {
                "todo" => {
                    if args.is_empty() {
                        println!("Usage: /todo <text>");
                    } else {
                        logbuch.todos.push(Todo {
                            timestamp: Utc::now().to_rfc3339(),
                            description: args.to_string(),
                            done: false,
                            sessions: Vec::new(),
                        });
                        save_logbuch(&path, &logbuch)?;
                    }
                }
                "list" | "ls" => list_todos(&logbuch),
                "toggle" => {
                    if args.is_empty() {
                        list_todos(&logbuch);
                    } else if let Some(actual_idx) = resolve_display_index(&logbuch, args) {
                        logbuch.todos[actual_idx].done = !logbuch.todos[actual_idx].done;
                        let mark = if logbuch.todos[actual_idx].done {
                            "[x]"
                        } else {
                            "[ ]"
                        };
                        println!("Toggled: {mark} {}", logbuch.todos[actual_idx].description);
                        save_logbuch(&path, &logbuch)?;
                    } else {
                        println!("Invalid index.");
                    }
                }
                "help" => print_help(),
                "quit" | "q" => break,
                _ => println!("Unknown command: /{cmd}"),
            }
        }
    }

    Ok(())
}
