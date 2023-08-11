use crate::front::hosting;

pub struct Customer {
    pub name: String,
}

pub enum CustomerStatus {
    Success,
    Failure
}

impl Customer {
    pub fn eat_at_a_restaurant(self) -> Option<CustomerStatus> {
        println!("I'm going to a restaurant.\n");
        hosting::add_to_waitlist(self);

        Some(CustomerStatus::Success)
    }
}