use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

static LISTS_DIR: &str = "lists";
static EXT: &str = ".json";
static DONE: char = '✓';
static NOT_DONE: char = '✕';

pub fn create_list(list_name: &str) -> io::Result<()> {
    let path = get_list_path(&list_name)?;

    // check if the file already exists
    if path.exists() {
        Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("List \"{}\" already exists.", list_name)
        ))
    } else {
        File::create(path)?;
        println!("Created list \"{}\".", list_name);
        Ok(())
    }
}

fn get_list_path(list_name: &str) -> io::Result<PathBuf> {
    let dir = PathBuf::from(LISTS_DIR);
    fs::create_dir_all(&dir)?; // make sure directory exists
    let filename = format!("{}{}", list_name, EXT);
    let path: PathBuf = dir.join(&filename);
    Ok(path)
}

pub fn delete_list(list_name: &str) -> io::Result<()> {
    match get_existing_list_path(list_name) {
        Some(path) => {
            fs::remove_file(&path)?;
            println!("Deleted list \"{}\".", list_name);
            Ok(())
        }
        None => {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("List \"{}\" was not found.", list_name),
            ))
        }
    }
}

fn get_existing_list_path(list_name: &str) -> Option<PathBuf> {
    let dir = PathBuf::from(LISTS_DIR);
    let filename = format!("{}{}", list_name, EXT);
    let path: PathBuf = dir.join(&filename);
    if path.exists() { Some(path) } else { None }
}

pub fn list_lists() -> io::Result<()> {
    let dir = PathBuf::from(LISTS_DIR);
    fs::create_dir_all(&dir)?;

    let entries = fs::read_dir(&dir)?;

    let mut found_any = false;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                println!("- {}", file_stem);
                found_any = true;
            }
        }
    }

    if !found_any {
        println!("No lists found.");
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    done: bool
}

pub fn print_list(list_name: &str) -> io::Result<()> {
    let tasks = get_tasks(&list_name)?;
    if tasks.len() > 0 {
        for (i, task) in tasks.iter().enumerate() {
            let fancy_done = if task.done { DONE } else { NOT_DONE };
            println!("{}: {} {}", i, fancy_done, task.title);
        }
    } else {
        println!("List \"{}\" is empty.", list_name)
    }
    Ok(())
}

fn get_tasks(list_name: &str) -> io::Result<Vec<Task>> {
    let path = get_existing_list_path(list_name)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("List \"{}\" not found.", list_name)))?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks: Vec<Task> = match serde_json::from_reader(reader) {
        Ok(t) => t,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into())
    };
    Ok(tasks)
}

fn save_tasks(list_name: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let path = get_existing_list_path(list_name)
           .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("List \"{}\" not found.", list_name)))?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}

pub fn add_task(list_name: &str, task: &str) -> io::Result<()> {
    let mut tasks = get_tasks(&list_name)?;
    tasks.push(Task {
        title: task.to_string(),
        done: false
    });
    save_tasks(list_name, &tasks)?;
    println!("Added \"{}\" to list \"{}\".", task, list_name);
    Ok(())
}

pub fn remove_task(list_name: &str, idx: usize) -> io::Result<()> {
    let mut tasks = get_tasks(&list_name)?;
    if tasks.len() > 0 {
        let title = tasks[idx].title.clone();
        tasks.remove(idx);
        save_tasks(list_name, &tasks)?;
        println!("Removed \"{}\" from list \"{}\".", title, list_name);
    } else {
        println!("List \"{}\" is empty.", list_name)
    }
    Ok(())
}

pub fn change_task_status(list_name: &str, idx: usize) -> io::Result<()> {
    let mut tasks = get_tasks(&list_name)?;
    if tasks.len() == 0 {
        println!("List \"{}\" is empty.", list_name)
    } else if idx >= tasks.len() {
        println!("Selected index is out of range.");
    } else {
       tasks[idx].done = !tasks[idx].done;
       save_tasks(list_name, &tasks)?;
       println!("Changed status for \"{}\" in \"{}\".", tasks[idx].title, list_name);
    }
    Ok(())
}

pub fn clear_tasks(list_name: &str) -> io::Result<()> {
    let tasks: Vec<Task> = Vec::new();
    save_tasks(&list_name, &tasks)?;
    println!("List \"{}\" cleared.", list_name);
    Ok(())
}
