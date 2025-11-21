use uuid::Uuid;

use serde_json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked { reason: String },
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: Uuid,
    title: String,
    description: String,
    priority: TaskPriority,
    status: TaskStatus,
}

impl Task {
    fn new(title: String, description: String, priority: TaskPriority, status: TaskStatus) -> Task {
        Task {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            status,
        }
    }

    fn display(&self) {
        println!("ID: {} | {} | {:?}", self.id, self.title, self.priority);
        println!("Status: {:?}", self.status);
        println!("Description: {}\n", self.description);
    }

    fn start(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Complete;
    }

    fn block(&mut self, reason: String) {
        self.status = TaskStatus::Blocked { reason: (reason) }
    }

    fn is_complete(&mut self) {
        self.status = TaskStatus::Complete;
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {

    pub fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new() 
        }
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        priority: TaskPriority,
        status: TaskStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // If file already exists
        if Path::new("data/tasks.json").exists() {
            let data = fs::read_to_string("data/tasks.json")?;
            self.tasks = serde_json::from_str(&data)?;
        }

        // Create and add new task
        let task = Task::new(title, description, priority, status);
        self.tasks.push(task);

        // Store tasks
        let json = serde_json::to_string_pretty(&self.tasks)?;
        
        let mut file = File::create("data/tasks.json")?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn list_tasks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new("data/tasks.json").exists() {
            let data = fs::read_to_string("data/tasks.json")?;
            self.tasks = serde_json::from_str(&data)?;
        }
        println!("Tasks: {:?}", self.tasks);
        Ok(())
    }
}
