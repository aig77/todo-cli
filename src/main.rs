mod todo_operations;
use clap::{Parser, Subcommand};
use todo_operations::{
    create_list, delete_list, list_lists, print_list, add_task, remove_task, change_task_status, clear_tasks
};

// A CLI Todo List written in Rust
#[derive(Parser, Debug)]
#[command(name="todo")]
#[command(about = "A simple CLI todo list app", long_about = None)]
struct TodoApp {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new todo list
    Create {
        /// Name of the list
        name: String,
    },
    /// Delete an existing list
    Delete {
        /// Name of the list
        name: String,
    },
    /// List all existing todo lists
    List,
    /// Prints a todo list
    Print {
       /// Name of the list
       name: String,
    },
    /// Add a task to a list
    Add {
        /// Name of the list
        name: String,
        /// Task to add
        task: String
    },
    /// Remove a task from a list at index
    Remove {
        /// Name of the list
        name: String,
        /// Index of the task to remove
        index: usize
    },
    /// Change status of task from a list at index
    Change {
        /// Name of the list
        name: String,
        /// Index of the task to change
        index: usize
    },
    /// Remove all tasks from a list
    Clear {
        /// Name of the list
        name: String
    }
}

fn main() {
    let app = TodoApp::parse();

    match app.command {
        Command::Create { name } => {
            if let Err(e) = create_list(&name) {
                eprintln!("{}", e);
            }
        }
        Command::Delete { name } => {
            if let Err(e) = delete_list(&name) {
                eprintln!("{}", e);
            }
        }
        Command::List => {
            if let Err(e) = list_lists() {
                eprintln!("{}", e);
            }
        }
        Command::Print { name } => {
            if let Err(e) = print_list(&name) {
                eprintln!("{}", e);
            }
        }
        Command::Add { name, task } => {
            if let Err(e) = add_task(&name, &task) {
                eprintln!("{}", e);
            }
        }
        Command::Remove { name, index } => {
            if let Err(e) = remove_task(&name, index) {
                eprintln!("{}", e);
            }
        }
        Command::Change { name, index } => {
            if let Err(e) = change_task_status(&name, index) {
                eprintln!("{}", e);
            }
        }
        Command::Clear { name } => {
            if let Err(e) = clear_tasks(&name) {
                eprintln!("{}", e);
            }
        }
    }
}
