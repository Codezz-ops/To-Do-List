use std::io::{self, Write};

fn add(tasks: &mut Vec<String>) {
    print!("Enter new task: ");
    io::stdout().flush().unwrap();
    let mut new_task = String::new();
    io::stdin().read_line(&mut new_task).expect("Failed to read line!");
    let task_name = format!("{} -- Active", new_task.trim());
    tasks.push(task_name.to_string());
    println!("Task added!");
}

fn help() {
    println!("Commands      Description");
    println!("  add         Adds new task");
    println!("  end         Ends tasks");
    println!("  list        List all tasks");
    println!("  quit        Quits & prints out all tasks");
    println!("  clear       Clears any task non-active");
}

fn clear(tasks: &mut Vec<String>) {
    let original_count = tasks.len();
    tasks.retain(|task| !task.ends_with(" -- Ended"));
    let removed_count = original_count - tasks.len();
    if removed_count > 0 {
        println!("Removed {} task(s) that were marked as ended.", removed_count);
    } else {
        println!("No tasks to remove.");
    }
}

fn end(tasks: &mut Vec<String>) {
    print!("Enter task to end: ");
    io::stdout().flush().unwrap();
    let mut ended_task = String::new();
    io::stdin().read_line(&mut ended_task).expect("Failed to read line!");
    let ended_task = ended_task.trim();

    let index = tasks.iter().position(|t| t.starts_with(ended_task));
    match index {
        Some(i) => {
            tasks[i] = format!("{} -- Ended", ended_task);
            println!("Task '{}' marked as ended.", ended_task);
        },
        None => println!("Task '{}' not found!", ended_task),
    }
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        print!("Enter a command: ");

        let mut user_input = String::new();

        io::stdout().flush().expect("Failed to flush line out");
        
        io::stdin().read_line(&mut user_input).expect("Failed to read line!");

        match user_input.trim() {
            "help" => help(),
            "add" => add(&mut tasks),
            "end" => end(&mut tasks),
            "clear" => clear(&mut tasks),
            "quit" => break,
            "list" => {
                for task in &tasks {
                    println!("{}", task);
                }
            }
            _ => println!("Invailed command, try again."),
        }
    }

    println!("Final Task List");
    for task in &tasks {
        println!("{}", task);
    }
}
