use std::rc::Rc;

fn main() {
    let text = Rc::new("Ayyy LmAo".to_string());
    let query = Rc::new("lmao".to_string());

    println!("{:?}", contains(text.clone(), query.clone()));
    println!("{:?}", contains_case(text.clone(), query.clone()));
}

fn contains(text: Rc<String>, query: Rc<String>) -> Vec<String> {
    let mut arr: Vec<String> = vec![];
    let q = query.to_string();

    for line in text.lines() {
        if line.contains(&q) {
            arr.push(line.to_string());
        }
    }

    return arr;
}

fn contains_case(text: Rc<String>, query: Rc<String>) -> Vec<String> {
    let lower_text = Rc::new(text.to_lowercase());
    let lower_qry = Rc::new(query.to_lowercase());
    
    contains(lower_text, lower_qry)
}