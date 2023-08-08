fn main() {
    let input: String = String::from("Testing string slices");

    let test1: &str = first_word(&input);
    let test2: &str = first_word(&input[8..]);

    let test3: String = input.chars().rev().collect();
    let test3: String = first_word(&test3).chars().rev().collect();

    println!("{} {} {}", test1, test2, test3);
}

fn first_word(input: &str) ->  &str {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return &input[0..i];
        }
    }

    return input;
}
