use std::collections::HashMap;
use std::io;
fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    // add_department(&mut departments, String::from("Marketing"));
    // add_employee(&mut departments, String::from("Marketing"), String::from("Amy"));
    main_menu(departments);
    //println!("{:?}", departments);
}
fn add_department(location: &mut HashMap<String, Vec<String>>, department_name: &String) {
    location.entry((&department_name).to_string()).or_insert_with(Vec::new);
}

fn add_employee(location: &mut HashMap<String, Vec<String>>, department: &String, employee_name: &String) -> InAction {
    if let Some(employees) = location.get_mut(&(department).to_string()) {
        employees.push((employee_name).to_string());
        return InAction::Continue
    } else {
        return InAction::Error
    }
}

fn get_employees(location: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all_employees: Vec<String> = Vec::new();
    for (_id, employee_list) in location {
        for item in employee_list {
            all_employees.push(item.to_string());
        }
    }
    all_employees.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    all_employees
}

fn get_employees_in_department(location: &HashMap<String, Vec<String>>, deparement: &String) -> Option<Vec<String>> {
    let mut employees: Vec<String> = Vec::new();
    if let Some(loc) = location.get(deparement) {
        for item in location.get(deparement).expect("Error") {
            employees.push(item.to_string());
        }
        employees.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        return Option::Some(employees)
    } else {
        return Option::None
    }
}

fn get_all_departments(location: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all_departments: Vec<String> = Vec::new();
    for (id, _employee_list) in location {
        all_departments.push(id.to_string());
    }
    all_departments.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    all_departments
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
    AllDepts,
    Quit,
    Error,
}

enum InAction {
    Continue,
    Error,
}

fn main_menu(mut departments: HashMap<String, Vec<String>>) {
    println!("COMPANY EMPLOYEE MANAGER");
    println!("Menu:");
    println!("Enter 1 to add a department.");
    println!("Enter 2 to add an employee to a department.");
    println!("Enter 3 to list all employees.");
    println!("Enter 4 to list all employees in a department.");
    println!("Enter 5 to list all departments.");
    println!("Enter 6 to quit program.");
    match main_menu_ui_logic(&get_user_input()) {
        MenuAction::AddDept => {
            println!("\nEnter the department's name.");
            let depart = get_user_input();
            add_department(&mut departments, &depart);
            println!("A new department called {} has been added.\n", &depart);
            main_menu(departments);
        }
        MenuAction::AddEmpl => {
            println!("\nEnter the employee's name.");
            let name = get_user_input();
            println!("Enter the name of the employee's department.");
            let depart = get_user_input();
            match add_employee(&mut departments, &depart, &name) {
                InAction::Continue => {
                    println!("{} has been added to the {} department.\n", &name, &depart);
                }
                InAction::Error => {
                    println!("\nThis department doesn't exist. Consider adding a new deparement.\n");
                }
            }
            main_menu(departments);
        }
        MenuAction::GetAll => {
            println!("\n Following is a list of all employees.\n");
            for i in get_employees(&departments) {
                println!("{}", i);
            }
            println!("\n");
            main_menu(departments);
        }
        MenuAction::GetDept => {
            println!("\nEnter the department you wish to inspect.");
            let dept = get_user_input();
            if let Some(list) = get_employees_in_department(&departments, &dept) {
                println!("\nFollowing is a list of all employees in the {} department.\n", dept);
                for i in list {
                    println!("{}", i);
                }
                println!("\n");
            } else {
                println!("\nDepartment \"{}\" was invalid. Please try again.\n", dept);
            }
            main_menu(departments);
        }
        MenuAction::AllDepts => {
            println!("\nFollowing is a list of all departments.\n");
            for i in get_all_departments(&departments) {
                println!("{}", i);
            }
            println!("\n");
            main_menu(departments);
        }
        MenuAction::Quit => {
            println!("Quitting program.")
        }
        MenuAction::Error => {
            println!("\nInvalid input.\n");
            main_menu(departments);
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
        return MenuAction::AllDepts
    }else if input == "6" {
        return MenuAction::Quit
    } else {
        return MenuAction::Error
    }
}