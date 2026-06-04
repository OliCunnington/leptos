use leptos::prelude::*;

#[component]
pub fn OutOfOrderStreaming() {
    view!{
        <div>
            
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