use std::io;

fn main() {
    println!("Input your temp in F:");
    loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your temp in C is {}", c_to_f(temp));
        break;
    }
}

fn c_to_f(f: f64) -> f64 {
    ((f - 32.0) * 5.0 / 9.0).round()
}
