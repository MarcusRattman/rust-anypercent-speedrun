#[cfg(test)]
mod front_tests {
    use crate::{front::hosting::*, front::serving::*, customer::Customer, back::*};

    #[test]
    fn waitlist_test() {
        let customer = Customer {
            name: String::from("Oleg"),
        };

        let result = add_to_waitlist(customer);

        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn order_taken_test() {
        let table = Table::new(Customer { name: String::from("Oleg") });
        let result = take_order(table);
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn order_served_test() {
        let table = Table::new(Customer { name: String::from("Oleg") });
        let order = Order::new(Toast::Wheat, "Mango", Appetizer::Salad, table);
        let result = serve_order(&order);

        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn order_paid_test () {
        let table = Table::new(Customer { name: String::from("Oleg") });
        let order = Order::new(Toast::Wheat, "Mango", Appetizer::Salad, table);
        let result = take_payment(&order);

        assert_eq!(result.is_some(), true);
    }
}