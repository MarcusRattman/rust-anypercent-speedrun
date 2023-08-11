#[cfg(test)]
mod customer_tests {
    use crate::customer::*;

    #[test]
    fn eating_test() {
        let customer = Customer { name: String::from("Oleg") };
        let result = customer.eat_at_a_restaurant();

        assert_eq!(result.is_some(), true);
    }
}