use std::collections::HashMap;

fn main() {
    let mut list_of_ints = vec![1, 2, 3, 4, 5, 6, 7, 7];
    list_of_ints.sort();

    let median = list_of_ints[list_of_ints.len() / 2];

    println!("The median is {:?}", median);

    let mut mode_counter: HashMap<i32, i32> = HashMap::new();

    for i in &list_of_ints {
        let count = mode_counter.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut highest_value_number = list_of_ints[0];
    let mut highest_value_count = mode_counter[&highest_value_number];

    for (number, num_count) in &mode_counter {
        if num_count > &highest_value_count {
            highest_value_count = *num_count;
            highest_value_number = *number;
        }
    }

    println!("The mode is {:?}", highest_value_number);
}
