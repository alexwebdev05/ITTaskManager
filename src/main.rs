mod models;
mod storage;

use std::io::{self, Write};

use models::{TaskManager, TaskPriority, TaskStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                let mut title = String::new();
                let mut description = String::new();
                let priority: TaskPriority;
                let status = TaskStatus::Pending;

                loop {
                    print!("Enter a title: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut title)?;
                    if !title.trim().is_empty() {break}
                }
                
                loop {
                    print!("Enter a description: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut description)?;
                    if !description.trim().is_empty() {break}
                }

                let mut priority_option = String::new();
                println!("\n\nSelect the priority: ");
                println!(" 1: Critical");
                println!(" 2: High");
                println!(" 3: Medium");
                println!(" 4: Low\n");
                io::stdin().read_line(&mut priority_option)?;

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
                )?;
                println!("Task added correctly.")
            }
            "2" => {
                manager.list_tasks()?;
            }
            "3" => {
                let mut uuid = String::new();
                let status: TaskStatus;
                let mut status_option = String::new();

                print!("Enter task UUID: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut uuid)?;

                println!("Select new state: ");
                println!(" 1: Pending");
                println!(" 2: In Progress");
                println!(" 3: Blocked");
                println!(" 4: Complete\n");

                io::stdin().read_line(&mut status_option)?;

                match status_option.trim() {
                    "1" => status = TaskStatus::Pending,
                    "2" => status = TaskStatus::InProgress,
                    "3" => {
                        let mut reason = String::new();
                        print!("Enter a reason: ");
                        io::stdout().flush()?;
                        io::stdin().read_line(&mut reason)?;

                        status = TaskStatus::Blocked { reason: reason.trim().to_string() }
                    },
                    "4" => status = TaskStatus::Complete,
                    _ => status = TaskStatus::Pending,
                }

                match manager.change_state(uuid.trim().to_string(), status) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("{}", e)
                }
            }
            "4" => {
                let mut priority_option = String::new();
                let priority: TaskPriority;

                println!("\n\nSelect the priority: ");
                println!(" 1: Critical");
                println!(" 2: High");
                println!(" 3: Medium");
                println!(" 4: Low\n");
                io::stdin().read_line(&mut priority_option)?;

                match priority_option.trim() {
                    "1" => priority = TaskPriority::Critical,
                    "2" => priority = TaskPriority::High,
                    "3" => priority = TaskPriority::Medium,
                    "4" => priority = TaskPriority::Low,
                    _ => {
                        println!("You doesn't select an option and it is now Medium priority.");
                        priority = TaskPriority::Medium
                    }
                }

                manager.filter_priority(priority)
            }
            "5" => break ,
            _ => println!("Select an option!!!"),
        }
    }
    Ok(())
}
