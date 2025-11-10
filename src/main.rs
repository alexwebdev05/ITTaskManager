use uuid::Uuid;

fn main() {
    
    #[derive(Debug, Clone)]
    enum TaskPriority {
        Critical,
        High,
        Medium,
        Low
    }

    #[derive(Debug, Clone)]
    enum TaskStatus {
        Pending,
        InProgress,
        Blocked { reason: String },
        Complete
    }

    #[derive(Debug, Clone)]
    struct Task {
        id: Uuid,
        title: String,
        description: String,
        priority: TaskPriority,
        status: TaskStatus
    }

    impl Task {

        pub fn new(title: String, description: String, priority: TaskPriority, status: TaskStatus) {
            Task {
                id: Uuid::new_v4(),
                title,
                description,
                priority,
                status
            };
        }

        pub fn display(&self) {
            println!("ID: {} | {} | {:?}", self.id, self.title, self.priority);
            println!("Status: {:?}", self.status);
            println!("Description: {}\n", self.description);
        }

        pub fn start() {

        }

        pub fn complete() {

        }

        pub fn block() {

        }

        pub fn is_complete() {

        }

    }

}
