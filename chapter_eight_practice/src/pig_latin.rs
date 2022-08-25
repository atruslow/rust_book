use std::io;

fn main() {

    let mut word = String::new();

    loop {
        println!("Enter a Word!");
        io::stdin().read_line(&mut word).unwrap();
        word = word.replace("\n", "");
        match word.chars().next() {
            None => {
              println!("Not a valid word");
              continue
            },
            _ => break
        };
    }

    println!("Here's you pig latin word {:?}", pig_latin(&word));
}

fn pig_latin (word: &String) -> String {
    let vowles = ["a", "e", "i", "o", "u", "y"];

    let first_character = String::from(word.chars().next().expect("Not a valid word"));

    if vowles.contains(&first_character.as_str()) {
        format!("{}-{}ay", &word, &"h")
    } else {
        format!("{}-{}ay", &word[1..], &first_character)
    }

}
