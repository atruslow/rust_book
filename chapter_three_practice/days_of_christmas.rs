fn main() {
    let days_of_christmas = vec![
        "On the first day of Christmas",
        "On the second day of Christmas",
        "On the third day of Christmas",
        "On the fourth day of Christmas",
        "On the fifth day of Christmas",
        "On the sixth day of Christmas",
        "On the seventh day of Christmas",
        "On the eighth day of Christmas",
        "On the ninth day of Christmas",
        "On the tenth day of Christmas",
        "On the eleventh day of Christmas",
        "On the twelfth day of Christmas",
    ];

    let presents = vec![
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for verse_num in 0..12 {
        println!("{}\n", get_verse(verse_num, &days_of_christmas, &presents))
    }
}

fn get_verse(n: usize, days_of_christmas: &Vec<&str>, presents: &Vec<&str>) -> String {
    let day_of_christmas = days_of_christmas[n].to_string();
    let mut other_stuff: Vec<&str> = vec![];
    other_stuff.extend(presents[0..n + 1].iter().rev());
    if n > 0 {
        let index = other_stuff.len() - 1;
        other_stuff[index] = "And a partridge in a pear tree"
    }

    format!(
        "{}\nMy true love sent to me\n{}",
        day_of_christmas,
        other_stuff.join("\n")
    )
}
