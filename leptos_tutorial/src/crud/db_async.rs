use gloo_timers::future::TimeoutFuture;

#[derive(Clone)]
pub struct Product {
    key: String,
    name: String,
    description: String,
    price: f32,
    stock: i32,
    supplier: String
}


static mut prods : Vec<Product> = Vec::new();
// {
//     Product {
//         key: "aaa".to_string(),
//         name: "Apple".to_string(),
//         description: "Juicy apple".to_string(),
//         price: 0.99,
//         stock: 20,
//         supplier: "A".to_string()
//     },
//     Product {
//         key: "bbb".to_string(),
//         name: "Banana".to_string(),
//         description: "delicious banana".to_string(),
//         price: 1.99,
//         stock: 10,
//         supplier: "B".to_string()
//     },
//     Product {
//         key: "ccc".to_string(),
//         name: "Carrot".to_string(),
//         description: "crunchy carrot".to_string(),
//         price: 0.99,
//         stock: 20,
//         supplier: "C".to_string()
//     }
// };


pub async fn get_products() -> Vec<Product> {
    TimeoutFuture::new(1_000).await;
    // prods.clone()
    vec![]
}

pub async fn get_product(key: String) -> Option<Product>{
    TimeoutFuture::new(1_000).await;
    None
}

pub async fn add_product(p: Product) -> bool {
    TimeoutFuture::new(1_000).await;
    true
}

pub async fn update_stock(key: String, s: i32) -> bool {
    TimeoutFuture::new(1_000).await;
    true
}