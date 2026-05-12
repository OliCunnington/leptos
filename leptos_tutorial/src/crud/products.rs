use leptos::prelude::*;
use leptos::Params;
use leptos::task::spawn_local;
use leptos::html;
use leptos::logging::log;
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
    
    let dialog_ref : NodeRef<html::Dialog> = create_node_ref();
    let id = use_params_map().read().get("id");
    let p = LocalResource::new(move || db_async::get_product(id.as_ref().expect("some string").to_string()));

    view!{
        <UpdateStockDialog dialog_ref=dialog_ref res=p />
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || {
                p.get()
                    .map(|p| view! { 
                        <p>{p.clone().unwrap().description}</p> 
                        <p>{p.unwrap().price}</p>
                        <button>"Edit"</button>
                        <button
                            on:click=move |_| { dialog_ref.get().unwrap().show_modal(); }
                        >"Update stock"</button>
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn ProductsContainerFor() -> impl IntoView {

    // let (items, set_items) = signal(Vec::new());
    let dialog_ref : NodeRef<html::Dialog> = create_node_ref();

    // create_effect(move |_| {
    //     spawn_local(async move {
    //         let fetched = db_async::get_products().await;
    //         set_items.update(|list| *list = fetched);
    //     });
    // });
    
    let ps = LocalResource::new(|| async {
        db_async::get_products().await
    });

    view!{
        <div class="prod_wrapper">
            <p>"NAME | STOCK | SUPPLIER"</p>
            <button
                on:click=move |_| { dialog_ref.get().unwrap().show_modal(); }
            >"+"</button>
            // <button
            //     on:click=move |_| { ps.refetch() }
            // >"REFRESH" </button>
        </div>
        <AddProductDialog dialog_ref=dialog_ref res=ps />
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <ul class="prod_rows">
                <For
                    // each=move || items.get()
                    // key=|prod| prod.key.clone()
                    // let(child)
                    each = move || ps.get().unwrap_or_default()
                    key = |prod| prod.key.clone()
                    let(child)
                >
                    <ProductRowAlt prod=child />
                </For>
            </ul>
        </Suspense>
    }
}

#[component]
pub fn ProductRowAlt(prod: db_async::Product) -> impl IntoView {

    let params = use_params_map();
    let id = move || params.read().get("id");
    let key = prod.key.clone();
    let val = prod.key.clone();

    let del_prod = Action::new(|input: &String| {
        let input= input.to_owned();
        async move { db_async::delete_prod(input).await }
    });

    // let del_wrap = move || {
    //     del_prod.dispatch(prod.key.clone());
    // };

    view!{
        <li>
            <A href={move || 
                if id().unwrap_or_default() == val {
                    val.clone() + "/.."
                } else {
                    val.clone()
                }
            }>
                <div class="prod_row">
                    <p>{prod.name}</p>
                    <p>{prod.stock}</p>
                    <p>{prod.supplier}</p>
                    <Show
                        when=move || { id().unwrap_or_default() == key.clone() }
                        fallback= || view! {}
                    >
                        <button //on:click=move |_| {
                            //del_wrap();
                            // let k = key.clone();
                        //     del_prod.dispatch(key);
                        // }
                        >
                            "DELETE"
                        </button>
                    </Show>
                </div>
            </A>
            <Show
                when=move || { id().unwrap_or_default() == prod.key.clone() }
                fallback= || view! {}
            >
                <Outlet/>
            </Show>
        </li>
    }
}

#[component]
fn AddProductDialog(
    dialog_ref: NodeRef<html::Dialog>,
    res: LocalResource<Vec<db_async::Product>>
) -> impl IntoView {
    let key_ref: NodeRef<html::Input> = NodeRef::new();
    let name_ref: NodeRef<html::Input> = NodeRef::new();
    let desc_ref: NodeRef<html::Textarea> = NodeRef::new();
    let price_ref: NodeRef<html::Input> = NodeRef::new();
    let stock_ref: NodeRef<html::Input> = NodeRef::new();
    let supplier_ref: NodeRef<html::Input> = NodeRef::new();

    let add_prod = Action::new(|input: &db_async::Product| {
        let input= input.to_owned();
        async move { db_async::add_product(input).await }
    });

    view!{
        <dialog node_ref=dialog_ref class="modal">
            <form>
                <label for="key">
                    "Key: "
                    <input
                        node_ref=key_ref
                        name="key"
                        id="key"
                        type="text"
                        placeholder="Key"
                    />
                </label>
                <label for="name">
                    "Name: "
                    <input
                        node_ref=name_ref
                        name="name"
                        id="name"
                        type="text"
                        placeholder="Name"
                    />
                </label>
                <label for="desc">
                    "Description: "
                    <textarea
                        node_ref=desc_ref
                        name="desc"
                        id="desc"
                        placeholder="Description"
                    />
                </label>
                <label for="price">
                    "Price: "
                    <input
                        node_ref=price_ref
                        name="price"
                        id="price"
                        type="number"
                        placeholder="Price"
                    />
                </label>
                <label for="stock">
                    "Stock: "
                    <input
                        node_ref=stock_ref
                        name="stock"
                        id="stock"
                        type="number"
                        placeholder="Stock"
                    />
                </label>
                <label for="supplier">
                    "Supplier: "
                    <input
                        node_ref=supplier_ref
                        name="supplier"
                        id="supplier"
                        type="text"
                        placeholder="Supplier"
                    />
                </label>
                <div>
                    <input type="submit" on:click=move |ev| {
                        ev.prevent_default();
                        let p = match price_ref.get().expect("expected price").value().parse::<f32>() {
                            Ok(n) => n,
                            Err(e) => 0.0
                        };
                        let s = match stock_ref.get().expect("expected stock").value().parse::<i32>() {
                            Ok(n) => n,
                            Err(e) => 0
                        };
                        let prod = db_async::Product{
                            key: key_ref.get().expect("expected key").value(),
                            name: name_ref.get().expect("expected name").value(),
                            description: desc_ref.get().expect("expected desc").value(),
                            price: p,
                            stock: s,
                            supplier: supplier_ref.get().expect("expected supplier").value()
                        };

                        log!("PROD log: {:?}", prod);

                        add_prod.dispatch(prod);
                        
                        res.refetch();

                        dialog_ref.get().unwrap().close();
                    } value="Submit" />
                    <input type="reset" value="Reset" />
                    <button on:click=move |ev| {
                        ev.prevent_default();
                        dialog_ref.get().unwrap().close();
                    }>"Cancel"</button>
                </div>
            </form>
        </dialog>
    }
}

#[component]
fn UpdateStockDialog(
    dialog_ref: NodeRef<html::Dialog>,
    res: LocalResource<Option<db_async::Product>>
) -> impl IntoView {

    let stock_ref: NodeRef<html::Input> = NodeRef::new();

    view!{
        <dialog node_ref=dialog_ref class="modal">
            <form>
                <label for="stock">
                    "Stock: "
                    <input
                        node_ref=stock_ref
                        name="stock"
                        id="stock"
                        type="number"
                        placeholder="Stock"
                        value={res.get().unwrap_or_default().map(|p| p.stock)}
                    />
                </label>
                <div>
                    <input type="Submit" value="Submit"/>
                    <button on:click=move |ev| {
                        ev.prevent_default();
                        dialog_ref.get().unwrap().close();
                    }>"Cancel"</button> 
                </div>
            </form>
        </dialog>
    }
}