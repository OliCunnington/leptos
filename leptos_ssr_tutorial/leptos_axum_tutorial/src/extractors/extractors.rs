use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyQuery {
    foo: String,
}

#[server]
pub async fn axum_extract() -> Result<String, ServerFnError> {
    use axum::{extract::Query, http::Method};
    use leptos_axum::extract;

    let (method, query): (Method, Query<MyQuery>) = extract().await?;

    Ok(format!("{method:?} and {query:?}"))
}

#[component]
pub fn Extractors() -> impl IntoView {
    let ext = Resource::new(
        || (),
        |_| axum_extract() 
    );

    view!{
        <h2>"Extractor return:"</h2>
        <Suspense fallback=|| view!{<p>"Loading..."</p>}>
            <p>{ext}</p>
        </Suspense>
    }
}