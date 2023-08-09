use crate::{hosting::Table, back::{Order, Toast, Appetizer}};

pub fn take_order(table: Table) {
    let order = Order::new(Toast::Wheat, "mango", Appetizer::Soup, table);
    println!("Thanks for your order!\n");
    serve_order(order);
}

pub fn serve_order(order: Order) {
    println!("Enjoy your meal!\n");
    take_payment(order);
}

pub fn take_payment(order: Order) {
    println!("Thank you very much,\nPlease come again!\n");
}