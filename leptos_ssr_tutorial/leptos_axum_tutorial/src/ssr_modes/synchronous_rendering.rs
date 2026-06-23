use leptos::prelude::*;

mod blog_elements;

#[component]
pub fn SynchronousRendering() -> impl IntoView  {

    let comments = LocalResource::new(
        // async move {
        //     blog_elements::blog_posts::get_comments().await
        // }
        move || blog_elements::blog_posts::get_comments()
    );
    // let comments_row = comments.get().unwrap_or(
    //     view!{<li>"Loading...."</li>}
    // ).into_iter().map(|p| {
    //     view!{
    //         <li>
    //             <blog_elements::blog_posts::BlogPostComment(p) />
    //         </li>
    //     }
    // }).collect_view();

    let posts = LocalResource::new(
        // async move {
        //     blog_elements::blog_posts::get_posts().await
        // }
        move || blog_elements::blog_posts::get_posts()
    );

    // let posts_row = posts.get().unwrap_or(
    //     view!{<li>"Loading...."</li>}
    // ).into_iter().map(|p| {
    //     view!{
    //         <li>
    //             <blog_elements::blog_posts::BlogPost(p) />
    //         </li>
    //     }
    // }).collect_view();


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
                    <For
                    each = move || posts.get().unwrap_or_default()
                    key = |post| post.user.clone()
                    let(child)
                    >
                        <li>
                            <blog_elements::blog_posts::BlogPost(child) />
                        </li>
                    </For>
                </ul>
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    <For
                    each = move || comments.get().unwrap_or_default()
                    key = |comment| comment.user.clone()
                    let(child)
                    >
                        <li>
                            <blog_elements::blog_posts::BlogPostComment(child) />
                        </li>
                    </For>
                </ul>
            </Suspense>
        </div>
    }
}