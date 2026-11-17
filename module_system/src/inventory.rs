mod analytics;
pub mod management;

pub struct Inventroy {
    pub products: Vec<Product>,
}

pub struct Product {
    id: i16,
    name: String,
    price: i32,
}

impl Product {
    pub fn new(name: String, price: Option<i32>) -> Self {
        Product {
            id: 0,
            name,
            price: price.unwrap_or(-1),
        }
    }
}
