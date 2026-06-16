use leptos::prelude::*;

#[component]
pub fn SynchronousRendering() {

    let comments = LocalResource::new(
        async move {
            blog_elements::blog_posts::get_comments().await
        }
    );
    
    let posts = LocalResource::new(
        async move {
            blog_elements::blog_posts::get_posts().await
        }
    );


    // let (id, set_id) = create_signal
    // let comments = create_local_resource(
    //     //scope
    //     cx,
    //     //source (signal)
    //     move || id.get(),
    //     //fetcher
    //     move |id| {
    //         let id = id;
    //         async move {
    //             blog_elements::blog_posts::get_post(id).await
    //         }
    //     }
    // );


    view!{
        <div>
            <h2>"Posts"</h2>
            <Suspense fallback=|| (view!{<p>"Loading..."</p>})>
                
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| (view!{<p>"Loading..."</p>})>

            </Suspense>
        </div>
    }
}