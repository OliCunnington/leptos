use leptos::prelude::*;
// use leptos_axum::extract_with_state;
// use axum::extract::FromRef;

// Leptos provides its own method of dependency injection via context. 
// Context can often be used instead of State to provide shared server data

// let connection_pool = /* some shared state here */;

// let app = Router::new()
//     .leptos_routes_with_context(
//         &leptos_options,
//         routes,
//         move || provide_context(connection_pool.clone()),
//         {
//             let leptos_options = leptos_options.clone();
//             move || shell(leptos_options.clone())
//         },
//     )
    // etc.

// #[derive(FromRef, Debug, Clone)]
// pub struct MyData {
//     pub value: usize,
//     pub leptos_options: std::sync::LazyLock<LeptosOptions>,
// }

// static app_state : MyData = MyData {
//     value: 42,
//     leptos_options: std::sync::LazyLock::new(
//         || LeptosOptions::builder().output_name("test").build()
//     )
// };

// build our application with a route
// let app = Router::new()
//     .leptos_routes_with_context(
//         &app_state,
//         routes,
//         {
//             let app_state = app_state.clone();
//             move || provide_context(app_state.clone())
//         },
//         App,
//     )
//     .fallback(file_and_error_handler)
//     .with_state(app_state);

//  ...
// #[server]
// pub async fn uses_state() -> Result<(), ServerFnError> {
//     let state = expect_context::<MyData>();
//     let SomeStateExtractor(data) = extract_with_state::<(), _>(&state).await?;
//     // todo
//     Ok(())
// }


#[component]
pub fn States() -> impl IntoView {
    view!{
        <p>"PLACEHOLDER KEKW"</p>
    }
}