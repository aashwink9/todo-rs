use std::io::{self, Write};

pub struct Task {
    title: String,
    desc: String,
    status: bool
}

impl Task {
    pub fn new(title: String, desc: String) -> Task {
        Task {
            title,
            desc,
            status: false
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn status(&self) -> bool {
        self.status
    }

    pub fn flip_status(&mut self) {
        self.status = !self.status;
    }

    pub fn change_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn change_desc(&mut self, new_desc: String) {
        self.desc = new_desc;
    }
}


pub struct TaskFactory {
    tasks: Vec<Task>,
    len: u32
}

impl TaskFactory {
    pub fn new() -> TaskFactory {
        TaskFactory {
            tasks: Vec::new(),
            len: 0
        }
    }

    pub fn add_task(&mut self, title: String, desc: String) {
        self.len += 1;
        let new_task: Task = Task::new(desc, title);
        self.tasks.push(new_task);
    }

    pub fn get_task(&mut self, task_id: usize) -> Option<&mut Task> {
        self.tasks.get_mut(task_id-1)
    }

    pub fn view_task(&self, task_id: usize) -> Option<&Task> {
        self.tasks.get(task_id-1)
    }

    pub fn get_num_tasks(&self) -> u32 {
        self.len
    }

    pub fn del_task(&mut self, task_id: usize) {
        match self.len.try_into() {
            Ok(len) => {
                if task_id > 0 || task_id < len {
                    self.tasks.remove(task_id-1);
                    self.len -= 1;
                }
                else {
                    println!("Task not found, please try again!");
                }
            },
            Err(_) => println!("Something went wrong!"),
        } 
    }

}

fn main() {
    start_loop();
}

fn start_loop() {
    let mut task_fac = TaskFactory::new();
    let mut cmd = String::new();
    let mssg: String = String::from("
+==========================================================+
+|new|: Create a task for your todo list                   +
+|modify|: Modify an existing task for the to-do list      +
+|delete|: Delete a task for the to-do list                +
+|view|: View all tasks                                    +
+|quit|: Quit the to-do list simulation                    +
+|help|: Display this message again!                       +
+==========================================================+
");
    println!("Welcome to your to-do list! Here are is the menu:{mssg}");

    while cmd.trim() != "quit" {
        print!("Enter a command: ");
        io::stdout().flush().unwrap();
        cmd.clear();
        
        io::stdin()
        .read_line(&mut cmd)
        .expect("An unexpected error occured with the input");
        
        match cmd.trim() {
            "new" => handle_create(&mut task_fac),
            "modify" => handle_modify(&mut task_fac),
            "delete" => handle_delete(&mut task_fac),
            "view" => handle_view(&mut task_fac),
            "ls" => handle_view(&mut task_fac),
            "help" => println!("\nLost eh? Worry not, for here is the menu!:\n{mssg}"),
            "quit" => println!("Quitting your instance...."),
            _ => println!("Invalid operation, please try again!")
        }
    }
}

fn handle_create(task_fac: &mut TaskFactory) {
    let mut title = String::new();
    let mut desc = String::new();

    println!("\n************ NEW TASK ****************");
    print!("Enter title: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut title)
        .expect("An unexpected error occured with the input");

    print!("Enter description: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut desc)
        .expect("An unexpected error occured with the input");

    if title != "" && desc != "" {
        task_fac.add_task(title, desc);
        println!("Task created successfully!");
    }
    else {
        println!("Please enter some text!");
    }

    println!("**************************************\n");

}
fn handle_modify(task_fac: &mut TaskFactory) {
    let num_tasks = task_fac.get_num_tasks();
    if num_tasks > 0 {
        let mut id_str = String::new();
        print!("Enter the task ID to modify (to view all tasks and their IDs, enter 'view'): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut id_str)
            .expect("An unexpected error occured with the input");

        if id_str.trim() == "view" {
            handle_view(task_fac);
        }
        match id_str.trim().parse::<usize>() {
            Ok(id) => {
                match task_fac.get_task(id) {
                    Some(task) => {
                        let mut new_change = String::new();

                        print!("What would you like to modify about the task?
                        a) Change Title
                        b) Change Description
                        c) Change Status

                        Your Response (a/b/c): ");
                        io::stdout().flush().unwrap();

                        let mut response = String::new();

                        io::stdin()
                        .read_line(&mut response)
                        .expect("An unexpected error occured with the input");

                        match response.trim() {
                            "a" => {
                                print!("Enter the new title: ");
                                io::stdout().flush().unwrap();

                                io::stdin()
                                .read_line(&mut new_change)
                                .expect("An unexpected error occured with the input");

                                task.change_title(new_change);

                                println!("Title changed successfully!");
                            },
                            "b" => {
                                print!("Enter the new description: ");
                                io::stdout().flush().unwrap();

                                io::stdin()
                                .read_line(&mut new_change)
                                .expect("An unexpected error occured with the input");

                                task.change_desc(new_change);

                                println!("Description changed successfully!");

                            },
                            "c" => {
                                task.flip_status();
                                println!("Status changed successfully!");
                            },
                            _ => println!("Invalid option please try again")
                        };
                    }
                    None => println!("Task not found. Please try again")
                }
            },
            Err(_) => println!("Not a valid ID, please try again!")
        };
    }
    else { println!("No tasks created!"); }
}

fn handle_view(task_fac: &mut TaskFactory) {
    let num_tasks = task_fac.get_num_tasks();
    if num_tasks > 0 {
        for id in 1..(num_tasks+1) {
            match id.try_into() {
                Ok(task_id) => {
                    match task_fac.view_task(task_id) {
                        Some(task) => {
                            println!("============ TASK {} ============", id);
                            print!("Task Title: {}", task.title());
                            print!("Task Description: {}", task.desc());
        
                            if task.status() == true { println!("Task Status: Done", ); }
                            else { println!("Task Status: Not Done", ); }
        
                        },
                        None => println!("Something went wrong!!")
                    }
                },
                Err(_) => println!("Something went wrong!")
            }
        }
        print!("================================\n\n");
    }
    else {
        println!("No tasks created yet!!");
    }
}

fn handle_delete(task_fac: &mut TaskFactory) {
    let mut id_str = String::new();
    print!("Enter the task ID to delete (to view all tasks and their IDs, enter 'view'): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut id_str)
        .expect("An unexpected error occured with the input");

    if id_str.trim() == "view" {
        handle_view(task_fac);
    }
    match id_str.trim().parse::<usize>() {
        Ok(id) => {
            task_fac.del_task(id);
            println!("Task deleted successfully!");
        },
        Err(_) => println!("Please enter a valid ID!")
    }
}