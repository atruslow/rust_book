use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
    let n: i64 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut cache = HashMap::from([(0, 0), (1, 1)]);

    println!("The {} fibonacci number is {}", n, fibonacci(n, &mut cache))
}

fn fibonacci(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if !cache.contains_key(&n) {
        let last_num = fibonacci(n - 1, cache);
        let num_before = fibonacci(n - 2, cache);
        cache.insert(n, last_num + num_before);
    };
    cache[&n]
}
