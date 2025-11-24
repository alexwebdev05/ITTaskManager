use std::str::FromStr;

use uuid::Uuid;

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
        for task in self.tasks.iter() {
            task.display();
        }
        Ok(())
    }

    pub fn change_state(&mut self, uuid: String, new_state: TaskStatus) -> Result<String, Box<dyn std::error::Error>> {
        let uuid_parsed = Uuid::from_str(&uuid)?;
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == uuid_parsed) {
            task.status = new_state.clone();
            self.storage.save_tasks(&self.tasks)?;
            Ok(format!("Task with UUID {} has a new state of {:?}", uuid, new_state))
        } else {
            Err(format!("Task with UUID {} not found", uuid).into())
        }
    }

    pub fn filter_priority(&mut self, priority: TaskPriority) {
        for task in self.tasks.iter() {
            if task.priority == priority { task.display() }
        }
    }

}