use crate::{hosting::Table, back::{Order, Toast, Appetizer}};

pub enum OrderStatus {
    Ordered,
    Served,
    Paid
}

pub fn take_order(table: Table) -> Option<OrderStatus> {
    let order = Order::new(Toast::Wheat, "mango", Appetizer::Soup, table);
    println!("Thanks for your order!\n");
    serve_order(&order);
    Some(OrderStatus::Ordered)
}

pub fn serve_order(order: &Order) -> Option<OrderStatus> {
    println!("Enjoy your meal!\n");
    take_payment(order);
    Some(OrderStatus::Served)
}

pub fn take_payment(order: &Order) -> Option<OrderStatus> {
    println!("Thank you very much,\nPlease come again!\n");
    Some(OrderStatus::Paid)
}