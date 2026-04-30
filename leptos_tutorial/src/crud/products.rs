use leptos::prelude::*;
use leptos::Params;
use leptos_router::components::{Outlet, A};
use leptos_router::params::Params;
use leptos_router::hooks::use_params_map;

struct Product {
    key: String,
    name: String,
    description: String,
    price: f32,
    stock: i32,
    supplier: String
}

#[derive(Params, PartialEq)]
struct ProductParam {
    id: Option<String>,
}

#[component]
pub fn ProductRow(prod: Product, select: WriteSignal<String>) -> impl IntoView {
    let selected = use_context::<ReadSignal<String>>();

    view!{
        <li>
            <A href={prod.key.clone()} 
                // on:click= move |_| {
                // if selected.read().expect("Some string?").to_string() == prod.key {
                //     *select.write() = "None".to_string();
                // } else {
                //     *select.write() = prod.key.clone();
                // }
                //}
                >
                <div class="prod_row">
                    <p>{prod.name}</p>
                    <p>{prod.stock}</p>
                    <p>{prod.supplier}</p>
                    // need buttons... 
                </div>
            </A>
            // if selected ?
            <Show
                when=move || {selected.read().expect("Some string?").to_string() == prod.key}
                fallback= || view! {}
            >
                <Outlet/>
            </Show>
        </li>
    }
}

#[component]
pub fn ProductsContainer(prods: Vec<Product>) -> impl IntoView { //(create button, list of prods...)
    // let params = use_params::<ProductParam>();
    let params = use_params_map();
    let id = move || params.read().get("id");

    let (selection, set_selection) = signal(id().unwrap_or("None".to_string()));
    provide_context(selection);

    let prod_rows = prods.into_iter().map(|p| {
        view!{
            <ProductRow prod=p select=set_selection />
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

#[component]
pub fn ProductExpanded() -> impl IntoView{
    view!{
        <h1>"Placeholder description"</h1>
    }
}