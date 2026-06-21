use leptos::prelude::*;

#[component]
pub fn OutOfOrderStreaming() -> impl IntoView {
    view!{
        <div>
            <h2>"Posts"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    // {posts_row}
                </ul>
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    // {comments_row}
                </ul>
            </Suspense>
        </div>
    }
}
// <Routes fallback=|| "Not found.">
//     // We’ll load the home page with out-of-order streaming and <Suspense/>
//     <Route path=path!("") view=HomePage/>

//     // We'll load the posts with async rendering, so they can set
//     // the title and metadata *after* loading the data
//     <Route
//         path=path!("/post/:id")
//         view=BlogPost
//         ssr=SsrMode::Async
//     />
// </Routes>