use crate::customer::Customer;
use crate::front::serving;
use rand::Rng;

pub struct Table {
    number: i32,
    customer: Customer,
}

impl Table {
    fn new(customer: Customer) -> Self {
        Self {
            number: rand::thread_rng().gen_range(1..=100),
            customer: customer,
        }
    }
}

pub fn add_to_waitlist(customer: Customer) {
    println!("Good afternoon!\nPlease have a seat.\n");
    let table = Table::new(customer);

    serving::take_order(table);
}