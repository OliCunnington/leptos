use leptos::prelude::*;
use leptos::Params;
use leptos_router::params::Params;
use leptos_router::hooks::{
    use_params,
    use_query, 
    use_params_map, 
    use_query_map
};


#[derive(Params, PartialEq)]
struct ContactParams {
    id: Option<usize>,
}

#[derive(Params, PartialEq)]
struct ContactSearch {
    q: Option<String>,
}

#[component]
pub fn TypedExample() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let query = use_query::<ContactSearch>();

    // id: || -> usize
    let id = move || {
        // returns Memo<Result<T, _>>
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };
    let name = move || match id() {
        0 => "Alice",
        1 => "Bob",
        2 => "Steve",
        _ => "User not found.",
    };
    view!{
        <p>{name}</p>
    }
}


#[component]
pub fn UntypedExample() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    // id: || -> Option<String>
    // returns Memo<ParamsMap> (.../:id)
    let id = move || params.read().get("id").unwrap();
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };
    view!{
        <p>{name}</p>
    }
}