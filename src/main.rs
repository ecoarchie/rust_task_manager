use chrono::{DateTime, Local};

struct Task {
    name: String,
    description: String,
    priority: Priority,
    date_created: DateTime<Local>,
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            date_created: Local::now(),
        }
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

    fn remove_task(&mut self, name: String) -> Result<String, String> {
        if let Some(i) = self.find_task_idx(&name) {
                self.tasks.remove(i);
                Ok(format!("Task \"{}\" removed", name))
        } else {
            Err("Task not found".to_owned())
        }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }
}

fn main() {
    let task = Task::new(
        "First task".to_owned(),
        "My first task in rust task manager".to_owned(),
        Priority::High,
    );

    task.print_task();
}
