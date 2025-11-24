use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

impl Task {
    
    pub fn new(title: String, description: String, priority: TaskPriority, status: TaskStatus) -> Task {
        Task {
            id: Uuid::new_v4(),
            title,
            description,
            priority,
            status,
        }
    }

    pub fn display(&self) {
        println!("ID: {} | {} | {:?}", self.id, self.title, self.priority);
        println!("Status: {:?}", self.status);
        println!("Description: {}\n", self.description);
    }

    // pub fn start(&mut self) {
    //     self.status = TaskStatus::InProgress;
    // }

    // pub fn complete(&mut self) {
    //     self.status = TaskStatus::Complete;
    // }

    // pub fn block(&mut self, reason: String) {
    //     self.status = TaskStatus::Blocked { reason: (reason) }
    // }

    // pub fn is_complete(&mut self) {
    //     self.status = TaskStatus::Complete;
    // }
}