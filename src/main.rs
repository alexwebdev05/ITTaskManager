mod task_manager;

use std::io;

use task_manager::{TaskManager, TaskPriority, TaskStatus};

fn main() {
    // Set up task manager
    let mut manager = TaskManager::new();

    // Cli
    println!("##### TaskManager #####\n");
    println!("Choose one of those options:");
    println!(" 1: Create a new task");
    println!(" 2: List tasks");
    println!(" 3: Change task state");
    println!(" 4: Filter by priority");
    println!(" 5: Close");

    // Get response
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    match input.trim() {
        "1" => {
            let mut title = String::new();
            let mut description = String::new();
            let mut priority: TaskPriority;
            let status = TaskStatus::Pending;

            println!("Enter a title: ");
            io::stdin().read_line(&mut title);
            
            println!("Enter a description: ");
            io::stdin().read_line(&mut description);
            
            let mut priority_option = String::new();
            println!("Select the priority: ");
            println!(" 1: Critical");
            println!(" 2: High");
            println!(" 3: Medium");
            println!(" 4: Low");
            io::stdin().read_line(&mut priority_option);

            match priority_option.trim() {
                "1" => priority = TaskPriority::Critical,
                "2" => priority = TaskPriority::High,
                "3" => priority = TaskPriority::Medium,
                "4" => priority = TaskPriority::Low,
                _ => priority = TaskPriority::Medium,
            }

            manager.add_task(title.trim().to_string(), description.trim().to_string(), priority, status);
            manager.list_tasks();
        },
        _ => print!("Select an option")
    }
}
