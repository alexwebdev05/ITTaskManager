mod models;
mod storage;

use std::io::{self, Write};

use models::{TaskManager, TaskPriority, TaskStatus};

fn main() {
    // Set up task manager
    let mut manager = TaskManager::new();

    // Loop
    loop {
        // Cli
        println!("\n##### TaskManager #####\n");
        println!("Choose one of those options:");
        println!(" 1: Create a new task");
        println!(" 2: List tasks");
        println!(" 3: Change task state");
        println!(" 4: Filter by priority");
        println!(" 5: Close\n");

        // Get response
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                let mut title = String::new();
                let mut description = String::new();
                let priority: TaskPriority;
                let status = TaskStatus::Pending;

                print!("Enter a title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter a description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                let mut priority_option = String::new();
                println!("\n\nSelect the priority: ");
                println!(" 1: Critical");
                println!(" 2: High");
                println!(" 3: Medium");
                println!(" 4: Low\n");
                io::stdin().read_line(&mut priority_option).unwrap();

                match priority_option.trim() {
                    "1" => priority = TaskPriority::Critical,
                    "2" => priority = TaskPriority::High,
                    "3" => priority = TaskPriority::Medium,
                    "4" => priority = TaskPriority::Low,
                    _ => priority = TaskPriority::Medium,
                }

                manager.add_task(
                    title.trim().to_string(),
                    description.trim().to_string(),
                    priority,
                    status,
                ).unwrap();
                println!("Task added correctly.")
            }
            "2" => {
                manager.list_tasks().unwrap();
            }
            _ => print!("Select an option"),
        }
    }
}
