use gloo_timers::future::TimeoutFuture;
use std::sync::{LazyLock, Mutex};

#[derive(Clone)]
pub struct Product {
    pub key: String,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub stock: i32,
    pub supplier: String
}


static PRODS : LazyLock<Mutex<Vec<Product>>> = LazyLock::new(|| Mutex::new({
    let mut v = Vec::new();
    v.push(Product {
        key: "aaa".to_string(),
        name: "Apple".to_string(),
        description: "Juicy apple".to_string(),
        price: 0.99,
        stock: 20,
        supplier: "A".to_string()
    });
    v.push(Product {
        key: "bbb".to_string(),
        name: "Banana".to_string(),
        description: "delicious banana".to_string(),
        price: 1.99,
        stock: 10,
        supplier: "B".to_string()
    });
    v.push(Product {
        key: "ccc".to_string(),
        name: "Carrot".to_string(),
        description: "crunchy carrot".to_string(),
        price: 0.99,
        stock: 20,
        supplier: "C".to_string()
    });
    v
}));

pub async fn get_products() -> Vec<Product> {
    TimeoutFuture::new(1_000).await;
    PRODS.lock().unwrap().clone()
}

pub async fn get_product(key: String) -> Option<Product>{
    TimeoutFuture::new(1_000).await;
    for p in PRODS.lock().unwrap().clone().into_iter() {
        if p.key == key {
            return Some(p);
        }
    }
    None
}

pub async fn add_product(p: Product) -> bool {
    TimeoutFuture::new(1_000).await;
    PRODS.lock().unwrap().push(p);
    true
}

pub async fn update_stock(key: String, s: i32) -> bool {
    // TODO make this modify in place... not clone?
    // ugh
    TimeoutFuture::new(1_000).await;
    for mut p in PRODS.lock().unwrap().clone().into_iter() {
        if p.key == key {
            p.stock += s;
        }
    }
    true
}