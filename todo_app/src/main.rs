use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(StructOpt)]
#[structopt(name = "todo_app", about = "A simple TODO app")]
enum Command {
    Add { task: String },
    Remove { index: usize },
    List,
    Done { index: usize },
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    task: String,
    done: bool,
}

fn load_todos(path: &PathBuf) -> Vec<TodoItem> {
    if path.exists() {
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    } else {
        Vec::new()
    }
}

fn save_todos(path: &PathBuf, todos: &Vec<TodoItem>) {
    let data = serde_json::to_string(todos).expect("Unable to serialize data");
    fs::write(path, data).expect("Unable to write file");
}

fn main() {
    let path = PathBuf::from("todos.json");
    let mut todos = load_todos(&path);

    let opt = Command::from_args();
    match opt {
        Command::Add { task } => {
            todos.push(TodoItem { task, done: false });
            save_todos(&path, &todos);
            println!("Task added!");
        }
        Command::Remove { index } => {
            if index < todos.len() {
                todos.remove(index);
                save_todos(&path, &todos);
                println!("Task removed!");
            } else {
                println!("Invalid index");
            }
        }
        Command::List => {
            for (i, item) in todos.iter().enumerate() {
                let status = if item.done { "done" } else { "not done" };
                println!("{}: {} [{}]", i, item.task, status);
            }
        }
        Command::Done { index } => {
            if index < todos.len() {
                todos[index].done = true;
                save_todos(&path, &todos);
                println!("Task marked as done!");
            } else {
                println!("Invalid index");
            }
        }
    }
}
