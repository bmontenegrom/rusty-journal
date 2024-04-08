use std::{fs::OpenOptions, io::{Result, Seek, SeekFrom}, path::PathBuf};


use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds, Local};



#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task{
    pub fn new(text: String) -> Task{
        Task{
            text,
            created_at: Utc::now(),
        }
    }
}



pub fn add_task(journal_path: &PathBuf, task: Task)->Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = match serde_json::from_reader(&file){
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => panic!("An error occurred: {}", e),
    };

    file.seek(SeekFrom::Start(0))?;
    tasks.push(task);
    serde_json::to_writer(&file, &tasks)?;
    Ok(())
}