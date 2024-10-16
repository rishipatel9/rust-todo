use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::{fs, io::Write};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }
}

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple CLI for managing your to-dos", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        task: String,
    },
    List,
    Complete {
        id: usize,
    },
}

const FILE_PATH: &str = "todo.json";

fn load_todo() -> Vec<Todo> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_todo(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).expect("Error  serializing todos");
    let mut file = fs::File::create(FILE_PATH).expect("Error while creating file");
    file.write_all(json.as_bytes()).expect("Error while writing to file");
}

fn add_task(title: String) {
    let mut todos = load_todo();
    let id = todos.len() + 1;
    todos.push(Todo::new(id, title));
    save_todo(&todos);
    println!("Task  added successfully!");
}
fn list_task() {
    let todos = load_todo();
    if todos.is_empty() {
        println!("No tasks found.");
    } else {
        for todo in todos {
            println!("{}: {} [{}]", todo.id, todo.title, if todo.completed { "Done" } else { "Not Done" });
        }
    }
}

fn complete_task(id: usize) {
    let mut todos = load_todo();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        save_todo(&todos);
        println!("Task {} marked as completed!", id);
    } else {
        println!("Task with id {} not found.", id);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => add_task(task),
        Commands::List => list_task(),
        Commands::Complete { id } => complete_task(id),
    }
}
