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

// static LISTS_DIR: &str = "lists";
// static EXT: &str = ".json";
// static DONE: char = '✓';
// static NOT_DONE: char = '✕';

// fn get_list_path(list_name: &str) -> io::Result<PathBuf> {
//     let dir = PathBuf::from(LISTS_DIR);
//     fs::create_dir_all(&dir)?; // make sure directory exists
//     let filename = format!("{}{}", list_name, EXT);
//     let path: PathBuf = dir.join(&filename);
//     Ok(path)
// }

// fn get_existing_list_path(list_name: &str) -> Option<PathBuf> {
//     let dir = PathBuf::from(LISTS_DIR);
//     let filename = format!("{}{}", list_name, EXT);
//     let path: PathBuf = dir.join(&filename);
//     if path.exists() { Some(path) } else { None }
// }

// fn create_list(list_name: &str) -> io::Result<()> {
//     let path = get_list_path(&list_name)?;

//     // check if the file already exists
//     if path.exists() {
//         Err(Error::new(
//             ErrorKind::AlreadyExists,
//             format!("List \"{}\" already exists.", list_name)
//         ))
//     } else {
//         File::create(path)?;
//         println!("Created list \"{}\".", list_name);
//         Ok(())
//     }
// }

// fn delete_list(list_name: &str) -> io::Result<()> {
//     match get_existing_list_path(list_name) {
//         Some(path) => {
//             fs::remove_file(&path)?;
//             println!("Deleted list \"{}\".", list_name);
//             Ok(())
//         }
//         None => {
//             Err(Error::new(
//                 ErrorKind::NotFound,
//                 format!("List \"{}\" was not found.", list_name),
//             ))
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Task {
//     title: String,
//     done: bool
// }

// fn get_tasks(list_name: &str) -> io::Result<Vec<Task>> {
//     let path = get_existing_list_path(list_name)
//         .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("List \"{}\" not found.", list_name)))?;
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//     let tasks: Vec<Task> = match serde_json::from_reader(reader) {
//         Ok(t) => t,
//         Err(e) if e.is_eof() => Vec::new(),
//         Err(e) => return Err(e.into())
//     };
//     Ok(tasks)
// }

// fn save_tasks(list_name: &str, tasks: &Vec<Task>) -> io::Result<()> {
//     let path = get_existing_list_path(list_name)
//            .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("List \"{}\" not found.", list_name)))?;
//     let file = File::create(path)?;
//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, tasks)?;
//     Ok(())
// }

// fn print_list(list_name: &str) -> io::Result<()> {
//     let tasks = get_tasks(&list_name)?;
//     if tasks.len() > 0 {
//         for (i, task) in tasks.iter().enumerate() {
//             let fancy_done = if task.done { DONE } else { NOT_DONE };
//             println!("{}: {} {}", i, fancy_done, task.title);
//         }
//     } else {
//         println!("List \"{}\" is empty.", list_name)
//     }
//     Ok(())
// }

// fn add_task(list_name: &str, task: &str) -> io::Result<()> {
//     let mut tasks = get_tasks(&list_name)?;
//     tasks.push(Task {
//         title: task.to_string(),
//         done: false
//     });
//     save_tasks(list_name, &tasks)?;
//     println!("Added \"{}\" to list \"{}\".", task, list_name);
//     Ok(())
// }

// fn change_task_status(list_name: &str, idx: usize) -> io::Result<()> {
//     let mut tasks = get_tasks(&list_name)?;
//     if tasks.len() == 0 {
//         println!("List \"{}\" is empty.", list_name)
//     } else if idx >= tasks.len() {
//         println!("Selected index is out of range.");
//     } else {
//        tasks[idx].done = !tasks[idx].done;
//        save_tasks(list_name, &tasks)?;
//        println!("Changed status for \"{}\" in \"{}\".", tasks[idx].title, list_name);
//     }
//     Ok(())
// }

// fn remove_task(list_name: &str, idx: usize) -> io::Result<()> {
//     let mut tasks = get_tasks(&list_name)?;
//     if tasks.len() > 0 {
//         let title = tasks[idx].title.clone();
//         tasks.remove(idx);
//         save_tasks(list_name, &tasks)?;
//         println!("Removed \"{}\" from list \"{}\".", title, list_name);
//     } else {
//         println!("List \"{}\" is empty.", list_name)
//     }
//     Ok(())
// }
