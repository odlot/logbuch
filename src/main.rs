use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

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

fn run_session(
    logbuch: &mut Logbuch,
    config: &mut Config,
    todo_idx: usize,
    duration: u32,
    path: &PathBuf,
    config_path: &PathBuf,
) -> io::Result<()> {
    let now = Utc::now();
    let session = Session {
        begin: now.to_rfc3339(),
        end: None,
        duration,
        notes: Vec::new(),
        breaks: Vec::new(),
    };
    logbuch.todos[todo_idx].sessions.push(session);
    save_logbuch(path, logbuch)?;

    config.default_duration = duration;
    save_config(config_path, config)?;

    let session_idx = logbuch.todos[todo_idx].sessions.len() - 1;
    let mut end_time = now + chrono::Duration::minutes(duration as i64);
    let expired = Arc::new(AtomicBool::new(false));
    let todo_desc = logbuch.todos[todo_idx].description.clone();

    let exp_clone = expired.clone();
    let timer_end = end_time;
    let timer = thread::spawn(move || {
        loop {
            let remaining = timer_end.signed_duration_since(Utc::now());
            if remaining.num_seconds() <= 0 {
                exp_clone.store(true, Ordering::Relaxed);
                println!("\nSession complete! ({duration} min)");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let stdin = io::stdin();
    let mut input = String::new();
    let mut on_break = false;

    loop {
        let remaining = end_time.signed_duration_since(Utc::now());
        if remaining.num_seconds() <= 0 || expired.load(Ordering::Relaxed) {
            break;
        }
        let mins = remaining.num_minutes();
        let secs = remaining.num_seconds() % 60;
        if on_break {
            print!("  {mins:02}:{secs:02} (break) -- {todo_desc} > ");
        } else {
            print!("  {mins:02}:{secs:02} -- {todo_desc} > ");
        }
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }

        if expired.load(Ordering::Relaxed) {
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
                "stop" => break,
                "break" | "coffee" => {
                    if on_break {
                        println!("Already on break.");
                    } else if let Ok(break_mins) = args.parse::<u32>() {
                        on_break = true;
                        let break_begin = Utc::now();
                        end_time += chrono::Duration::minutes(break_mins as i64);
                        let brk = Break {
                            begin: break_begin.to_rfc3339(),
                            end: None,
                            duration: break_mins,
                        };
                        logbuch.todos[todo_idx].sessions[session_idx]
                            .breaks
                            .push(brk);
                        save_logbuch(path, logbuch)?;
                        println!("Break started ({break_mins} min). /continue to resume.");
                    } else {
                        println!("Usage: /break <duration>");
                    }
                }
                "continue" => {
                    if on_break {
                        on_break = false;
                        let break_idx = logbuch.todos[todo_idx].sessions[session_idx]
                            .breaks
                            .len()
                            - 1;
                        logbuch.todos[todo_idx].sessions[session_idx].breaks[break_idx].end =
                            Some(Utc::now().to_rfc3339());
                        save_logbuch(path, logbuch)?;
                        println!("Break ended. Session resumed.");
                    } else {
                        println!("Not on break.");
                    }
                }
                "todo" => {
                    if !args.is_empty() {
                        logbuch.todos.push(Todo {
                            timestamp: Utc::now().to_rfc3339(),
                            description: args.to_string(),
                            done: false,
                            sessions: Vec::new(),
                        });
                        save_logbuch(path, logbuch)?;
                    }
                }
                "list" | "ls" => list_todos(logbuch),
                "help" => print_help(),
                "quit" | "q" => {
                    if on_break {
                        let break_idx = logbuch.todos[todo_idx].sessions[session_idx]
                            .breaks
                            .len()
                            - 1;
                        logbuch.todos[todo_idx].sessions[session_idx].breaks[break_idx].end =
                            Some(Utc::now().to_rfc3339());
                    }
                    logbuch.todos[todo_idx].sessions[session_idx].end =
                        Some(Utc::now().to_rfc3339());
                    save_logbuch(path, logbuch)?;
                    expired.store(true, Ordering::Relaxed);
                    let _ = timer.join();
                    std::process::exit(0);
                }
                _ => println!("Unknown command: /{cmd}"),
            }
        } else {
            logbuch.todos[todo_idx].sessions[session_idx]
                .notes
                .push(Note {
                    timestamp: Utc::now().to_rfc3339(),
                    description: trimmed.to_string(),
                });
            save_logbuch(path, logbuch)?;
        }
    }

    if on_break {
        let break_idx = logbuch.todos[todo_idx].sessions[session_idx]
            .breaks
            .len()
            - 1;
        logbuch.todos[todo_idx].sessions[session_idx].breaks[break_idx].end =
            Some(Utc::now().to_rfc3339());
    }
    logbuch.todos[todo_idx].sessions[session_idx].end = Some(Utc::now().to_rfc3339());
    save_logbuch(path, logbuch)?;

    expired.store(true, Ordering::Relaxed);
    let _ = timer.join();
    Ok(())
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
    let mut config = load_config(&config_path)?;
    save_config(&config_path, &config)?;

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
                "start" => {
                    if args.is_empty() {
                        println!("Usage: /start <index>");
                    } else if let Some(actual_idx) = resolve_display_index(&logbuch, args) {
                        print!("Duration [{}]: ", config.default_duration);
                        io::stdout().flush()?;
                        let mut dur_input = String::new();
                        stdin.read_line(&mut dur_input)?;
                        let duration = if dur_input.trim().is_empty() {
                            config.default_duration
                        } else if let Ok(d) = dur_input.trim().parse::<u32>() {
                            d
                        } else {
                            println!("Invalid duration.");
                            continue;
                        };
                        run_session(
                            &mut logbuch,
                            &mut config,
                            actual_idx,
                            duration,
                            &path,
                            &config_path,
                        )?;
                    } else {
                        println!("Invalid index.");
                    }
                }
                "stop" => println!("No active session."),
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
