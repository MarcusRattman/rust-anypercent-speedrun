fn main() {
    let numarray: [i32; 5] = [0, 25, 3, 45, 96];
    let vector: Vec<i32> = vec![0, 25, 3, 45, 96];
    let chararray: [char; 5] = ['a', 'z', 'f', 'g', 'q'];

    largest(numarray);
    largest(chararray);
    largest(vector);

    let point = Point { x: 32, y: 12 };
    point.get();
}

fn largest<T>(array: T) where 
    T: IntoIterator, 
    T::Item: std::cmp::Ord + Default + std::fmt::Display {
        
    let mut lrg: T::Item = T::Item::default();

    array.into_iter().for_each(| element | {
        if element > lrg {
            lrg = element;
        }
    });

    println!("Largest for: {}", lrg);
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