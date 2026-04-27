use leptos::prelude::*;

struct Product {
    key: i32,
    name: String,
    description: String,
    price: f32,
    stock: i32,
    supplier: String
}

#[component]
pub fn ProductRow(prod: Product) -> impl IntoView {
    view!{
        // a href=:{prod.key}
        <div class="prod_row">
            <p>{prod.name}</p>
            <p>{prod.stock}</p>
            <p>{prod.supplier}</p>
        </div>
        // <Outlet/>
    }
}