use crate::customer::Customer;
use crate::front::serving;
use rand::Rng;

pub struct Table {
    number: i32,
    customer: Customer,
}

pub enum WaitlistStatus {
    Added,
    Failed,
}

impl Table {
    pub fn new(customer: Customer) -> Self {
        Self {
            number: rand::thread_rng().gen_range(1..=100),
            customer: customer,
        }
    }
}

pub fn add_to_waitlist(customer: Customer) -> Option<WaitlistStatus> {
    println!("Good afternoon!\nPlease have a seat.\n");
    let table = Table::new(customer);

    serving::take_order(table);

    Some(WaitlistStatus::Added)
}