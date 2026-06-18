use leptos::prelude::*;

#[component]
pub fn SynchronousRendering() -> impl IntoView  {

    let comments = LocalResource::new(
        async move {
            blog_elements::blog_posts::get_comments().await
        }
    );
    let comments_row = comments.into_iter().map(|p| {
        view!{
            <li>
                <blog_elements::blog_posts::BlogPostComment(p) />
            </li>
        }
    }).collect_view();

    let posts = LocalResource::new(
        async move {
            blog_elements::blog_posts::get_posts().await
        }
    );

    let posts_row = posts.into_iter().map(|p| {
        view!{
            <li>
                <blog_elements::blog_posts::BlogPost(p) />
            </li>
        }
    }).collect_view();


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
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    {posts_row}
                </ul>
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    {comments_row}
                </ul>
            </Suspense>
        </div>
    }
}