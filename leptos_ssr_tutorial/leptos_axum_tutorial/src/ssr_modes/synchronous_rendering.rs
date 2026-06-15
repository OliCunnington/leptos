use leptos::prelude::*;

#[component]
pub fn SynchronousRendering() {

    let (id, set_id) = create_signal
    // let comments = LocalResource::new()
    let comments = create_local_resource(
        //scope
        cx,
        //source (signal)
        move || id.get(),
        //fetcher
        move |id| {
            let id = id;
            async move {
                blog_elements::blog_posts::get_post(id).await
            }
        }
    );
    view!{
        <div>
            <Suspense fallback=|| (view!{<p>"Loading..."</p>})>
                // local resources? eh?
            </Suspense>
        </div>
    }
}