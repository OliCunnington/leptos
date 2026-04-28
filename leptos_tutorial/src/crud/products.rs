use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

struct Product {
    key: String,
    name: String,
    description: String,
    price: f32,
    stock: i32,
    supplier: String
}

#[component]
pub fn ProductRow(prod: Product) -> impl IntoView {
    view!{
        <li>
            <A href={prod.key} >
                <div class="prod_row">
                    <p>{prod.name}</p>
                    <p>{prod.stock}</p>
                    <p>{prod.supplier}</p>
                    // need buttons... 
                </div>
            </A>
            <Outlet/>
        </li>
    }
}

#[component]
pub fn ProductsContainer(prods: Vec<Product>) -> impl IntoView { //(create button, list of prods...)
    
    let prod_rows = prods.into_iter().map(|p| {
        view!{
            <ProductRow prod=p />
        }
    }).collect_view();

    view!{
        // needs a "wrapper" box for sorting, create button
        <div class="prod_wrapper">
        </div>
        <ul class="prod_rows">
            {prod_rows}
        // <For
        //     each=move || prods.get()
        //     key=|counter| counter.0
        //     children=move |(id, count)| {

        //     }
        // />

        </ul>
    }
}

pub fn Products() -> impl IntoView {
    let prods = vec![
        Product {
            key: "aaa".to_string(),
            name: "Apple".to_string(),
            description: "Juicy apple".to_string(),
            price: 0.99,
            stock: 20,
            supplier: "A".to_string()
        },
        Product {
            key: "bbb".to_string(),
            name: "Banana".to_string(),
            description: "delicious banana".to_string(),
            price: 1.99,
            stock: 10,
            supplier: "B".to_string()
        },
        Product {
            key: "ccc".to_string(),
            name: "Carrot".to_string(),
            description: "crunchy carrot".to_string(),
            price: 0.99,
            stock: 20,
            supplier: "C".to_string()
        }
    ];

    view! {
        <ProductsContainer prods=prods />
    }
}