use leptos::prelude::*;
use leptos::Params;
use leptos_router::components::{Outlet, A};
use leptos_router::params::Params;
use leptos_router::hooks::use_params_map;
use crate::crud::db_async;

// pub mod crud;

#[derive(Clone)]
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
                >
                <div class="prod_row">
                    <p>{prod.name}</p>
                    <p>{prod.stock}</p>
                    <p>{prod.supplier}</p>
                    // need buttons... 
                </div>
            </A>
            <Show
                when=move || { selected.read().expect("Some string?").to_string() == prod.key }
                fallback= || view! {}
            >
                <Outlet/>
            </Show>
        </li>
    }
}

#[component]
pub fn ProductsContainer(prods: Vec<Product>) -> impl IntoView { //(create button, list of prods...)
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
        <div class="prod_wrapper">
        <h2>"Collected view"</h2>
        </div>
        <ul class="prod_rows">
            {prod_rows}
        </ul>
    }
}

#[component]
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
        <ProductsContainer prods=prods.clone() />
        <br />
        <ProductsContainerFor/> // prods=prods />
    }
}

#[component]
pub fn ProductExpanded() -> impl IntoView{
    let id = use_params_map().read().get("id");
    let p = LocalResource::new(move || db_async::get_product(id));

    view!{
        <h1>"Placeholder description for "{id}</h1>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || {
                p.get()
                    .map(|p| view! { 
                        <p>{p.unwrap().description}</p> 
                        <p>{p.unwrap().price}</p>
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn ProductsContainerFor() -> impl IntoView {

    let p = LocalResource::new(move || db_async::get_products());

    view!{
        <div class="prod_wrapper">
            <h2>"For"</h2>
            <p>"some buttons..."</p>
        </div>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <ul class="prod_rows">
                <For
                    each=move || p.get().clone()
                    key=|prod| prod.get().key.clone()
                    let(child)
                >
                    <ProductRowAlt prod=child />
                </For>
            </ul>
        </Suspense>
    }
}

#[component]
pub fn ProductRowAlt(prod: Product) -> impl IntoView {

    let params = use_params_map();
    let id = move || params.read().get("id");
    let key = prod.key.clone();
    let val = prod.key.clone();

    view!{
        <li>
            <A href={move || 
                if id().unwrap_or_default() == key {
                    val.clone() + "/.."
                } else {
                    val.clone()
                }
            }>
                <div class="prod_row">
                    <p>{prod.name}</p>
                    <p>{prod.stock}</p>
                    <p>{prod.supplier}</p>
                </div>
            </A>
            <Show
                when=move || { id().unwrap_or_default() == prod.key }
                fallback= || view! {}
            >
                <Outlet/>
            </Show>
        </li>
    }
}