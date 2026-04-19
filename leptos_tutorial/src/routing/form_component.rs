use leptos::prelude::*;
use leptos::prelude::Resource;
use leptos_router::components::{Form, Route, Router, Routes};
use leptos_router::hooks::use_query_map;
use leptos_router::path;
use leptos_router::MatchNestedRoutes; 
use leptos_router::any_nested_route::IntoAnyNestedRoute;

#[component(transparent)]
pub fn Demo() -> impl MatchNestedRoutes + Clone {
    view! {
        // <Router>
        //     <h1><code>"<Form/>"</code></h1>
        //     <main>
                // <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                    <Route path=path!("/routing/form") view=FormExample/>
                // </Routes>
        //     </main>
        // </Router>
    }
    .into_inner()
    .into_any_nested_route()
}

#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let name = move || query.read().get("name").unwrap_or_default();
    let number = move || query.read().get("number").unwrap_or_default();
    let select = move || query.read().get("select").unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input
        <h2>"Automatic Submission"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name
                // this oninput attribute will cause the
                // form to submit on every input to the field
                oninput="this.form.requestSubmit()"
            />
            <input
                type="number"
                name="number"
                value=number
                oninput="this.form.requestSubmit()"
            />
            <select name="select"
                onchange="this.form.requestSubmit()"
            >
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}

async fn fetch_results() {
    // some async function to fetch our search results
}

#[component]
pub fn FormSearchExample() -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map();
    // search stored as ?q=
    let search = move || query.read().get("q").unwrap_or_default();
    // a resource driven by the search string
    let search_results = Resource::new(search, |_| fetch_results());

    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search
            oninput="this.form.requestSubmit()"
            />
            // <input type="submit"/> // replaced with oninput
        </Form>
        <Transition fallback=move || view! { <h1>"Not Found"</h1> }>
            /* render search results */
            // {todo!()}
            <p>"Some Result"</p>
        </Transition>   
    }
}