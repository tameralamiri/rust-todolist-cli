use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before reading.
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after reading.
    Ok(tasks) // Return the tasks read from the file.
}

pub fn add_task(journal_file: PathBuf, task: Task) -> Result<()> {
    // Open the journal file in read write create mode.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_file)?; // ? will return early if there is an error.
    // Consume the file content as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Write the modified Task Lists back to the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_file: PathBuf, task_position: usize) -> Result<()> {
    // Open the journal file in read write mode.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_file)?;

    // Consume the file content as a vector of tasks.
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Try to Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // Write the modified task list back into the file.
    file.set_len(0)?; // Truncating the file before writing to it before the new file will be shorter than the old one. with this line we insure writing on a blank file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_file: PathBuf) -> Result<()> {
    // Open the journal file in read only mode.
    let file = OpenOptions::new().read(true).open(journal_file)?;
    // Parse the file and collect the tasks.
    let tasks: Vec<Task> = collect_tasks(&file)?;

    //Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty.");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task); 
        }
    }
    Ok(())
}