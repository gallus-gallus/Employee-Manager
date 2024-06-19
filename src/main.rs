use std::collections::HashMap;
use std::io;
fn main() {
    let mut departements: HashMap<String, Vec<String>> = HashMap::new();
    // add_departement(&mut departements, String::from("Marketing"));
    // add_employee(&mut departements, String::from("Marketing"), String::from("Amy"));
    main_menu(departements);
    //println!("{:?}", departements);
}
fn add_departement(location: &mut HashMap<String, Vec<String>>, departement_name: &String) {
    location.entry((&departement_name).to_string()).or_insert_with(Vec::new);
}

fn add_employee(location: &mut HashMap<String, Vec<String>>, departement: &String, employee_name: &String) {
    if let Some(employees) = location.get_mut(&(departement).to_string()) {
        employees.push((employee_name).to_string());
    } else {
        println!("This departement doesn't exist. Consider adding a new deparement.");
    }
}

fn get_employees(location: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all_employees: Vec<String> = Vec::new();
    for (id, employee_list) in location {
        for item in employee_list {
            all_employees.push(item.to_string());
        }
    }
    all_employees.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    all_employees
}

fn get_employees_in_departement(location: &HashMap<String, Vec<String>>, deparement: &String) -> Vec<String> {
    let mut employees: Vec<String> = Vec::new();
    for item in location.get(deparement).expect("Error") {
        employees.push(item.to_string());
    }
    employees.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    employees
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

enum MenuAction {
    AddDept,
    AddEmpl,
    GetAll,
    GetDept,
    Quit,
    Error,
}

enum InAction {
    Contune (String),
    Menu,
}

fn main_menu(mut departements: HashMap<String, Vec<String>>) {
    println!("COMPANY EMPLOYEE MANAGER");
    println!("Menu:");
    println!("Enter 1 to add a departement.");
    println!("Enter 2 to add an employee to a departement.");
    println!("Enter 3 to list all employees.");
    println!("Enter 4 to list all employees in a departement.");
    println!("Enter 5 to quit program.");
    match main_menu_ui_logic(&get_user_input()) {
        MenuAction::AddDept => {
            departements = add_departement_ui(departements);
            main_menu(departements);
        }
        MenuAction::AddEmpl => {
            departements = add_employee_ui(departements);
            main_menu(departements);
        }
        MenuAction::GetAll => {
            get_employees_ui(&departements);
            main_menu(departements);
        }
        MenuAction::GetDept => {
            get_employees_in_departement_ui(&departements);
            main_menu(departements)
        }
        MenuAction::Quit => {}
        MenuAction::Error => {
            println!("\nInvalid input.\n");
            main_menu(departements);
        }
    }
}

fn main_menu_ui_logic(input: &str) -> MenuAction {
    if input == "1" {
        return MenuAction::AddDept
    }else if input == "2" {
        return MenuAction::AddEmpl
    }else if input == "3" {
        return MenuAction::GetAll
    }else if input == "4" {
        return MenuAction::GetDept
    }else if input == "5" {
        return MenuAction::Quit
    } else {
        return MenuAction::Error
    }
}

fn add_departement_ui(mut departements: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    println!("\nEnter the departement's name.");
    let depart = get_user_input();
    add_departement(&mut departements, &depart);
    println!("A new departement called {} has been added.\n", &depart);
    departements
}

fn add_employee_ui(mut departements: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    println!("\nEnter the employee's name.");
    let name = get_user_input();
    println!("Enter the name of the employee's departement.");
    let depart = get_user_input();
    add_employee(&mut departements, &depart, &name);
    println!("{} has been added to the {} departement.\n", &name, &depart);
    departements
}

fn get_employees_ui(departments: &HashMap<String, Vec<String>>) {
    println!("\n Following is a list of all employees.\n");
    for i in get_employees(&departments) {
        println!("{}", i);
    }
    println!("\n");
}

fn get_employees_in_departement_ui(departments: &HashMap<String, Vec<String>>) {
    println!("\nEnter the departement you wish to inspect.");
    let dept = get_user_input();
    println!("\n Following is a list of all employees in the {} departement.\n", dept);
    for i in get_employees_in_departement(&departments, &dept) {
        println!("{}", i);
    }
    println!("\n");
}