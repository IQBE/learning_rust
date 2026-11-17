// Note: you can't use `use crate::inventory` here because lib.rs and main.rs are different crates.
// In this file, `crate::` would reference the binary crate, while we are interested in the
// library crate.
use module_system::inventory::{Inventroy, Product};

fn main() {
    let apple = Product::new(String::from("Apple"), Some(2));
    let pear = Product::new(String::from("Pear"), Some(3));

    let my_inv = Inventroy {
        products: vec![apple, pear],
    };
}
