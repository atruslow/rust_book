use regex::Regex;
use std::collections::HashMap;
use std::io;

pub fn departments() {
    let mut department_employee: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("What would you like to do?");
        println!("1. input an employee");
        println!("2. retrieve department");
        println!("3. retrieve whole company");

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: i32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            1 => add_employee(&mut department_employee),
            2 => {
                get_department(&mut department_employee);
                break;
            }
            3 => {
                get_company(&mut department_employee);
                break;
            }
            _ => continue,
        }
    }
}

fn add_employee(department_employee: &mut HashMap<String, Vec<String>>) {
    println!("Who would you like to add?");

    let mut person_str = String::new();
    io::stdin().read_line(&mut person_str).unwrap();

    let re = Regex::new(r"Add (\w+) to (\w+)").unwrap();

    let cap = re.captures(&person_str).unwrap();

    let name = String::from(&cap[1]);
    let dept = String::from(&cap[2]);

    department_employee
        .entry(String::from(&dept))
        .or_insert(Vec::new());
    department_employee.get_mut(&dept).unwrap().push(name);
    for (key, value) in department_employee {
        println!("{:?}: {:?}", key, value);
    }
}

fn get_department(dept_hash: &HashMap<String, Vec<String>>) {}

fn get_company(dept_hash: &HashMap<String, Vec<String>>) {}
