use std::{
    fmt,
    fs::{File, OpenOptions},
    io::{Error, ErrorKind, Result, Seek, SeekFrom},
    path::PathBuf,
};

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => panic!("An error occurred: {}", e),
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn add_task(journal_path: &PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(&file, &tasks)?;
    Ok(())
}

pub fn complete_task(journal_path: &PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid task ID"));
    }
    tasks.remove(task_position - 1);
    file.set_len(0)?;
    let _ = serde_json::to_writer(file, &tasks);
    Ok(())
}

pub fn list_tasks(journal_path: &PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;
    if tasks.is_empty() {
        println!("Tasks list is empty!");
    } else {
        let mut order = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }
    Ok(())
}
