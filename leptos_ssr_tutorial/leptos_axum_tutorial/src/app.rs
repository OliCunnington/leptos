use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    StaticSegment,
    SsrMode,
    path
};
use crate::ssr_modes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/style/main.scss"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <nav>
                <a href="">"Home"</a>
                <a href="/actionform">"Actionform"</a>
                <a href="/async_closures">"Async Closures Reference sheet"</a>
                <a href="/extractors">"Extractors"</a>
                <a href="/hydration_bugs">"Common Hydration Bugs"</a>
                <a href="/progressive_enhancment">"Progressive enhancment and Graceful Degredation"</a>
                <a href="/ssr_modes">"SSR Modes"</a>
                <a href="/server_functions">"Server Functions"</a>
            </nav>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/actionform") view=HomePage/>
                    <Route path=path!("/async_closures") view=HomePage/>
                    <Route path=path!("/extractors") view=HomePage/>
                    <Route path=path!("/hydration_bugs") view=HomePage/>
                    <Route path=path!("/progressive_enhancment") view=HomePage/>
                    <ParentRoute path=path!("/ssr_modes") view=ssr_modes::page::SSRModes>
                        <Route 
                            path=path!("/async") 
                            view=ssr_modes::async_rendering::AsynchronousRendering
                            ssr=SsrMode::Async
                        />
                        <Route 
                            path=path!("/blocking") 
                            view=ssr_modes::blocking::Blocking
                            ssr=SsrMode::PartiallyBlocked 
                        />
                        <Route
                            path=path!("/inorder") 
                            view=ssr_modes::in_order::InOrderStreaming 
                            ssr=SsrMode::InOrder
                        />
                        <Route 
                            path=path!("/outoforder") 
                            view=ssr_modes::out_of_order::OutOfOrderStreaming 
                        />
                        <Route 
                            path=path!("/partial") 
                            view=ssr_modes::partially_blocking::PartiallyBlocking 
                            ssr=SsrMode::PartiallyBlocked 
                        />
                        <Route 
                            path=path!("/synchronous") 
                            view=ssr_modes::synchronous_rendering::SynchronousRendering 
                        />
                        <Route 
                            path=path!("/*any") 
                            view=|| view! { <h1>"Not Found"</h1> }
                        />
                    </ParentRoute>
                    <Route path=path!("/server_functions") view=HomePage/>
                    // <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
