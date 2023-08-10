use std::fmt::{Debug, Display};
use std::cmp::Ord;

fn main() {
    let numarray: [i32; 5] = [0, 25, 3, 45, 96];
    let vector: Vec<i32> = vec![0, 25, 3, 45, 96];
    let chararray: [char; 5] = ['a', 'z', 'f', 'g', 'q'];

    largest(numarray);
    largest(chararray);
    largest(vector);
    largest(String::from("String").chars());
    largest("String slice uwu".chars());

    let point = Point { x: 32, y: 12 };
    point.get();
}

fn largest<T>(collection: T) where 
    T: IntoIterator + Debug,
    T::Item: Ord + Default + Display {
        
    let mut lrg: T::Item = T::Item::default();

    println!("\n{:?}", collection);

    collection.into_iter().for_each(| element | {
        if element > lrg {
            lrg = element;
        }
    });

    println!("Largest: {}", lrg);
}

struct Point<T: std::fmt::Display> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    fn get(&self) {
        println!("x: {} ; y: {}", self.x, self.y);
    }
}