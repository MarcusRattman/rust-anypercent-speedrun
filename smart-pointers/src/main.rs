fn main() {
    let text = Box::new("Ayyy LmAo".to_string());
    let query = Box::new("lmao".to_string());

    println!("{:?}", contains(text.clone(), query.clone()));
    println!("{:?}", contains_case(text.clone(), query.clone()));
}

fn contains(text: Box<String>, query: Box<String>) -> Vec<String> {
    let mut arr: Vec<String> = vec![];
    let q = query.to_string();

    for line in text.lines() {
        if line.contains(&q) {
            arr.push(line.to_string());
        }
    }

    return arr;
}

fn contains_case(text: Box<String>, query: Box<String>) -> Vec<String> {
    let lower_text = Box::new(text.to_lowercase());
    let lower_qry = Box::new(query.to_lowercase());
    
    contains(lower_text, lower_qry)
}