use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked { reason: String },
    Complete,
}

#[derive(Debug, Clone)]
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
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        priority: TaskPriority,
        status: TaskStatus,
    ) {
        self.tasks
            .push(Task::new(title, description, priority, status));
    }

    pub fn list_tasks(&self) {
        println!("Tasks: {:?}", self.tasks);
    }

    pub fn find_task_mut(&mut self, id: Uuid) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn list_by_priority(&self) {
        let mut critical: Vec<&Task> = Vec::new();
        let mut high: Vec<&Task> = Vec::new();
        let mut medium: Vec<&Task> = Vec::new();
        let mut low: Vec<&Task> = Vec::new();

        for task in self.tasks.iter() {
            match task.priority {
                TaskPriority::Critical => critical.push(task),
                TaskPriority::High => high.push(task),
                TaskPriority::Medium => medium.push(task),
                TaskPriority::Low => low.push(task),
            }
        }

        println!("Tasks: {:?} {:?} {:?} {:?}", critical, high, medium, low)
    }
}
