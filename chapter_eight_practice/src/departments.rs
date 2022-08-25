
use std::io;
use std::collections::HashMap;

pub fn departments() {

    let department_employee: HashMap<String, Vec<String>> = HashMap::new();

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
            1 => add_employee(&department_employee),
            2 => {get_department(&department_employee); break},
            3 => {get_company(&department_employee); break},
            _ => continue,
        }

    }
}

fn add_employee (dept_hash: &HashMap<String, Vec<String>>) {

    println!("Who would you like to add?");
    
    let mut person_str = String::new();
    io::stdin().read_line(&mut person_str).unwrap();

    let 


}

fn get_department (dept_hash: &HashMap<String, Vec<String>>) {

}

fn get_company (dept_hash: &HashMap<String, Vec<String>>) {

}