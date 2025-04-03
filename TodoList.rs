use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};

const FILE_NAME: &str = "todo.txt";

fn main() {
    println!("Simple Rust To-Do List");
    loop {
        println!("\n1. Add Task\n2. View Tasks\n3. Remove Task\n4. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        match choice.trim() {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => remove_task(),
            "4" => break,
            _ => println!("Invalid option, try again!"),
        }
    }
}

fn add_task() {
    let mut task = String::new();
    println!("Enter task:");
    io::stdin().read_line(&mut task).expect("Failed to read input");
    let mut file = OpenOptions::new().append(true).create(true).open(FILE_NAME).expect("Cannot open file");
    writeln!(file, "{}", task.trim()).expect("Failed to write to file");
    println!("Task added!");
}

fn view_tasks() {
    let file = File::open(FILE_NAME).unwrap_or_else(|_| File::create(FILE_NAME).unwrap());
    let reader = io::BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if let Ok(task) = line {
            println!("{}. {}", i + 1, task);
        }
    }
}

fn remove_task() {
    view_tasks();
    println!("Enter task number to remove:");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read input");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };
    let file = File::open(FILE_NAME).expect("Cannot open file");
    let reader = io::BufReader::new(file);
    let tasks: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    if index == 0 || index > tasks.len() {
        println!("Invalid task number!");
        return;
    }
    let new_tasks: Vec<String> = tasks.into_iter().enumerate().filter(|(i, _)| *i + 1 != index).map(|(_, task)| task).collect();
    let mut file = File::create(FILE_NAME).expect("Cannot recreate file");
    for task in new_tasks {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }
    println!("Task removed!");
}
