fn main() {
    for i in 1..=100 {
        let mut output: String = String::new();

        if i % 3 == 0 {
            output += "Fizz";
        }

        if i % 5 == 0 {
            output += "Buzz";
        }

        if output.is_empty() {
            output += "Not divisible";
        }

        println!("{i}: {output}")
    }
}
