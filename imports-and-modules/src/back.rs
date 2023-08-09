use crate::hosting::Table;

pub enum Appetizer {
    Soup,
    Salad,
}

pub enum Toast {
    Rye,
    Wheat,
}

struct Breakfast {
    toast: Toast,
    seasonal_fruit: String,
}

pub struct Order {
    breakfast: Breakfast,
    appetizer: Appetizer,
    table: Table,
}

impl Order {
    pub fn new(toast: Toast, fruit: &str, app: Appetizer, table: Table) -> Self {
        Self {
            breakfast: Breakfast { toast, seasonal_fruit: String::from(fruit) },
            appetizer: app,
            table,
        }
    }
}