use gloo_timers::future::TimeoutFuture;

#[derive(Clone)]
struct Product {
    key: String,
    name: String,
    description: String,
    price: f32,
    stock: i32,
    supplier: String
}

async pub fn get_products() -> Vec<Product> {
    vec![]
}

async pub fn get_product(key: String) -> Option<Product>{
    None
}

async pub fn add_product(p: Product) {

}

async pub fn update_stock(key: String, s: i32) {
    
}