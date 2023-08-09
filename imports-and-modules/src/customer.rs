use crate::front::hosting;

pub struct Customer {
    pub name: String,
}

impl Customer {
    pub fn eat_at_a_restaurant(self) {
        println!("I'm going to a restaurant.\n");
        hosting::add_to_waitlist(self);
    }
}