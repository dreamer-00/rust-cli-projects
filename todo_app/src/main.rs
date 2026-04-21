use std::io;
fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("\n To-Do App");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Delete Task");
        println!("4. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim(){
            "1" => {
                println!("Enter task: ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed");
                let task = task.trim();
                if task.is_empty() {
                    println!("Task cannot be empty!");
                    continue;
                }
                tasks.push(task.trim().to_string());
                println!("Task successfully added!");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks yet!");
                }
                else{
                    println!("\n Your Tasks: ");
                    for (i, task) in tasks.iter().enumerate() {
                        println!("[{}] {}", i, task);
                    }
                }   
            }
            "3" => {
                println!("Enter index to delete: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed");
                let index: usize = match index.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Number");
                        continue;
                    }
                };
                if index >= tasks.len(){
                    println!("Index out of range");
                    continue;
                }
                
                println!("Are you sure you want to delete \"{}\" ? (y/n)", tasks[index]);
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).expect("Failed");
                if confirm.trim() == "y" {
                    tasks.remove(index);
                    println!("Task removed!");
                }
                else{
                    println!("Cancelled");
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
