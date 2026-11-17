pub mod inventory;

fn test() {
    let my_prod = inventory::Product::new(String::from("Apple"), Some(2));

    use inventory::management;
    management::add_product(my_prod);
    // management::clear_inventory()   // NOT ALLOWED: This is a private function
}
