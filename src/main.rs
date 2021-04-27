use std::collections::HashMap;
use std::io;
use std::io::Write;

use ansi_term::Colour;

fn main() {
    let mut mode: String = String::from("");

    println!("What mode to go?\n{} - Pig Lating Conversor\n{} - Mean, Median and Mode of Vec\n{} - Company Employee Management",
             Colour::Purple.paint("1"),
             Colour::Purple.paint("2"),
             Colour::Purple.paint("3"));

    io::stdin()
        .read_line(&mut mode)
        .expect("Couldn't read mode");

    let mode = mode.trim()
        .parse().expect("Couldn't parse");

    match mode {
        1 => pig_latin_string_conversor(),
        2 => get_mean_median_and_mode(),
        3 => company_employees_management(),
        _ => panic!("Mode {} does not exist", mode)
    }
}

#[allow(dead_code)]
fn company_employees_management() {
    let mut state = EmployeeManagementState::new();

    while state.quit == false {
        let mut cmd: String = String::from("");

        println!("Type {} if you want to add a new employee to a department, {} if you want to list employees or {} to quit",
                 Colour::Cyan.underline().paint("add"),
                 Colour::Cyan.underline().paint("list"),
                 Colour::Cyan.underline().paint("quit"));

        io::stdin()
            .read_line(&mut cmd)
            .expect("Couldn't read command");


        state.command = Command::from(&cmd);
        match state.command {
            Some(_) => {
                state.run_command();
            }
            None => continue
        }
    }
}

struct EmployeeManagementState {
    map: HashMap<String, Vec<String>>,
    quit: bool,
    command: Option<Command>,
}

impl EmployeeManagementState {
    fn new() -> EmployeeManagementState {
        EmployeeManagementState {
            map: HashMap::new(),
            quit: false,
            command: None,
        }
    }

    fn run_command(&mut self) {
        match self.command {
            Some(Command::Add) => self.add_employee(),
            Some(Command::ListDepartment) => self.list_department(),
            Some(Command::ListCompany) => self.list_company(),
            Some(Command::Quit) => self.quit(),
            None => panic!("No command inferred")
        }
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn add_employee(&mut self) {
        let mut employee: String = String::from("");
        let mut department = String::from("");

        print!("Type the employee's name: ");
        io::stdout().flush().ok();
        io::stdin()
            .read_line(&mut employee)
            .expect("Could not get employee's name");
        employee = employee.trim().to_string();

        print!("Type the department's name ({}): ", Colour::Purple.italic().paint(self.get_departments_names().join(", ")));
        io::stdout().flush().ok();
        io::stdin()
            .read_line(&mut department)
            .expect("Could not get department's name");
        department = department.trim().to_string();

        println!("Adding employee {} to the department {}\n", Colour::Green.underline().paint(&employee), Colour::Purple.underline().paint(&department));
        let vec = self.map.entry(department).or_insert(Vec::new());
        vec.push(employee);
        vec.sort();
    }

    fn list_department(&self) {
        let mut department = String::from("");

        print!("Type the department's name ({}): ", Colour::Purple.italic().paint(self.get_departments_names().join(", ")));
        io::stdout().flush().ok();
        io::stdin()
            .read_line(&mut department)
            .expect("?");
        department = department.trim().to_string();

        match self.map.get(&department) {
            Some(employees) => println!("Employees from {}: {}\n", Colour::Green.paint(department), Colour::Purple.paint(employees.join(", "))),
            None => println!("There are no employees in the {} department\n", Colour::Cyan.paint(department))
        }
    }

    fn get_departments_names(&self) -> Vec<&str> {
        self.map.keys().map(|k| &**k).collect::<Vec<&str>>()
    }

    fn list_company(&self) {
        println!("Employees in the company:");
        for entry in &self.map {
            println!("{}: {}\n",
                     Colour::Green.paint(entry.0),
                     Colour::Purple.paint(entry.1.join(", ")));
        }
    }
}

enum Command {
    Add,
    ListDepartment,
    ListCompany,
    Quit,
}

impl Command {
    fn from(cmd: &String) -> Option<Command> {
        let cmd_lowercase = cmd.to_lowercase();
        let cmd_word: &str = cmd_lowercase.split_whitespace().collect::<Vec<&str>>()[0];

        match cmd_word {
            "add" => Option::from(Command::Add),
            "quit" => Option::from(Command::Quit),
            "list" => {
                let mut next_cmd: String = String::from("");
                println!("Type {} to get all employees in the company or {} to get all employees in one department",
                         Colour::Cyan.paint("company"),
                         Colour::Cyan.paint("department"));

                io::stdin()
                    .read_line(&mut next_cmd)
                    .expect("Couldn't read next command");

                next_cmd = next_cmd.to_lowercase();
                let next_cmd: &str = next_cmd.split_whitespace().collect::<Vec<&str>>()[0];

                match next_cmd {
                    "department" => Option::from(Command::ListDepartment),
                    "company" => Option::from(Command::ListCompany),
                    _ => {
                        println!("Sub-command {} for Command {} not found\n", Colour::Red.paint(next_cmd), Colour::Cyan.paint("list"));
                        None
                    }
                }
            }
            _ => {
                println!("Command {} not found\n", Colour::Red.paint(cmd_word));
                None
            }
        }
    }
}

#[allow(dead_code)]
fn get_mean_median_and_mode() {
    let number_list = vec![1, 2, 3, 3, 4, 5, 6];

    let mean = number_list.iter().fold(0, |a, b| a + b) / number_list.len();
    let median = number_list[(number_list.len() - 1) / 2];

    let mut number_count_map: HashMap<_, _> = HashMap::new();
    for number in number_list {
        let count = number_count_map.entry(number).or_insert(0);
        *count += 1;
    }

    let mode = number_count_map
        .iter()
        .max_by_key(|entry| entry.1)
        .unwrap().0;

    println!(
        "mean: {} | median: {} | mode: {}",
        mean, median, mode
    );
}

#[allow(dead_code)]
fn pig_latin_string_conversor() {
    let pig_latin = to_pig_latin(String::from("apple"));

    println!("{}", pig_latin);
}

#[allow(dead_code)]
fn to_pig_latin(str: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let first_letter = str.chars().next().unwrap();

    if vowels.contains(&first_letter) {
        str + "-hay"
    } else {
        format!("{}-{}ay", &str[1..], first_letter)
    }
}