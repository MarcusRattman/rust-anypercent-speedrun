#![allow(dead_code)]
#![allow(unused_variables)]

use crate::front::hosting;
mod front;
mod back;
pub mod customer;

mod tests {
    mod front_tests;
    mod back_tests;
    mod customer_tests;
}