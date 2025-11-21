use super::task::{Task, TaskPriority, TaskStatus};
use crate::storage::JsonManager;

pub struct TaskManager {
    storage: JsonManager,
    tasks: Vec<Task>
}

impl TaskManager {

    pub fn new() -> TaskManager {
        let storage = JsonManager::new("data/tasks.json".to_string());
        let tasks = storage.load_tasks().unwrap_or_else(|_| Vec::new());
        TaskManager {
            storage,
            tasks
        }
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        priority: TaskPriority,
        status: TaskStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create and add new task
        let task = Task::new(title, description, priority, status);
        self.tasks.push(task);

        // Store tasks
        self.storage.save_tasks(&self.tasks)?;

        Ok(())
    }

    pub fn list_tasks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Tasks: {:?}", self.tasks);
        Ok(())
    }
}