use imports_and_modules::customer::Customer;
fn main() {
    let customer = Customer { name: String::from("Jake") };
    customer.eat_at_a_restaurant();
}
