use std::fmt::{Display, Formatter};
use std::io::{stdin, Result, stdout, Write};


struct Task {
    name: String,
    completed: bool
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {})", self.name, self.completed)
    }
}

fn main() -> Result<()> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Choose an Option:");
        println!(" 1: Add Task");
        println!(" 2: List Tasks");
        println!(" 3: Quit");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        let int_input = buffer.trim().parse::<i32>().unwrap();
        match int_input {
            1 => {
                print!("Task: ");
                stdout().flush().expect("Wrong");
                let mut input_task = String::new();
                stdin().read_line(&mut input_task)?;
                let task = Task {
                    name: input_task,
                    completed: false
                };
                println!("Adding task: {}", &task);
                tasks.push(task);
            },
            2 => {
                for task in tasks.iter() {
                    println!("task: {}", task)
                }
            },
            _ => break
        }
    }
    Ok(())
}