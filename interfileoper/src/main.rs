use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            let status = Command::new("ls")
                .arg(directory_path)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                println!("Error listing directory");
            }
        }

        FileOperation::Display(file_path) => {
            let status = Command::new("cat")
                .arg(file_path)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error displaying file");
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File '{}' created successfully.", file_path);
            } else {
                println!("Failed to create file");
            }
        }

        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File '{}' removed successfully", file_path);
            } else {
                println!("Failed to remove file");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error executing pwd");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = get_input("Enter your choice (0-5):");

        let operation = match choice.as_str() {
            "1" => {
                let path = get_input("Enter directory path:");
                Some(FileOperation::List(path))
            }

            "2" => {
                let path = get_input("Enter file path:");
                Some(FileOperation::Display(path))
            }

            "3" => {
                let path = get_input("Enter file path:");
                let content = get_input("Enter content:");
                Some(FileOperation::Create(path, content))
            }

            "4" => {
                let path = get_input("Enter file path:");
                Some(FileOperation::Remove(path))
            }

            "5" => Some(FileOperation::Pwd),

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please try again.");
                None
            }
        };

        if let Some(op) = operation {
            perform_operation(op);
        }
    }
}