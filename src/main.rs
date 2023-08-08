use std::{io::{Write, BufReader}, path::Path, fs::File};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// use serde_json;

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    date_created: DateTime<Local>,
    date_updated: Option<DateTime<Local>>,
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            date_created: Local::now(),
            date_updated: None,
        }
    }

    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter task name: ").unwrap();
        let description = ConsoleManager::input("Enter task description: ").unwrap();
        let priority = match ConsoleManager::input("Enter task priority: ")
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("Incorrect priority input, set to Low");
                Priority::Low
            }
        };
        Self::new(
            name,
            description,
            priority,
        )
    }

    fn print_task(&self) {
        println!(
            "{} | {} | {}\n{}\n",
            self.name,
            self.priority.to_string(),
            self.date_created.format("%d-%m-%Y %H:%M:%S"),
            self.description,
        )
    }
}

#[derive(Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn find_task_idx(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|t| name == t.name)
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(i) = self.find_task_idx(name) {
            self.tasks.remove(i);
            Ok(format!("Task \"{}\" removed", name))
        } else {
            Err("Task not found".to_owned())
        }
    }

    fn edit_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(i) = self.find_task_idx(&name) {
            let task_to_update = self.tasks.get_mut(i).unwrap();
            let updated_task = Task::new_from_console();
            task_to_update.name = updated_task.name;
            task_to_update.description = updated_task.description;
            task_to_update.priority = updated_task.priority;
            task_to_update.date_updated = Some(Local::now());

            Ok(format!(
                "Task \"{}\" successfully updated at {}",
                name, task_to_update.date_updated.unwrap().format("%d-%m-%Y %H:%M:%S")
            ))
        } else {
            Err("Task not found".to_owned())
        }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn store_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => {
                    return Err(format!("Error creating file {}", err))
                }
            };
            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data stored successfully".to_owned()),
                Err(err) => Err(format!("Error saving to file: {}", err)),
            }
            
        } else {
            Err("File {} already exists".to_owned())
        }
    }

    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => {
                    return Err(format!("Error creating file {}", err))
                }
            };
            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!("Error reading data: {}", err))
                }
            };

            Ok("".to_owned())
        } else {
            Err("File {} doesn't exist".to_owned())
        }
    }
}

struct ConsoleManager {
    task_manager: TaskManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            task_manager: TaskManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Edit task".to_owned(),
                "Find task".to_owned(),
                "Remove task".to_owned(),
                "Print tasks".to_owned(),
                "Save tasks to file".to_owned(),
                "Read tasks from file".to_owned(),
            ],
        }
    }

    fn print_menu(&self) {
        for (index, option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush();

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("Enter commmand index: ") {
            Ok(command) => match command.as_str() {
                "1" => {
                    self.task_manager.add_task(Task::new_from_console());
                }
                "2" => {
                    let name = match Self::input("Enter task name to edit: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting input name: {}", err);
                            return;
                        }
                    };
                    match self.task_manager.edit_task(&name) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => println!("{}", msg),
                    }
                }
                "3" => {
                    let name = match Self::input("Enter task name to find: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting input name: {}", err);
                            return;
                        }
                    };
                    match self.task_manager.find_task_idx(&name) {
                        Some(index) => self.task_manager.tasks.get(index).unwrap().print_task(),
                        None => println!("Task does not exist"),
                    }
                }
                "4" => {
                    let name = match Self::input("Enter task name to remove: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting input name: {}", err);
                            return;
                        }
                    };
                    match self.task_manager.remove_task(&name) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => println!("{}", msg),
                    }
                }
                "5" => {
                    self.task_manager.print_tasks();
                }
                "6" => {
                    let filename = match Self::input("Enter file name to store data: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting input name: {}", err);
                            return;
                        }
                    };

                    match self.task_manager.store_to_file(&filename) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => println!("{}", msg),
                    }
                }
                "7" => {
                    let filename = match Self::input("Enter file name to read data: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting name: {}", err);
                            return;
                        }
                    };

                    match self.task_manager.read_from_file(&filename) {
                        Ok(msg) => println!("{}", msg),
                        Err(msg) => println!("{}", msg),
                    }
                }

                _ => println!("Unknown command"),
            },
            Err(err) => println!("Error getting user input - {}", err),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
