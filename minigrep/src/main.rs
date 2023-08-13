use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = fs::read_to_string("poem.txt").expect("No such file");
    println!("{}", file)
}
