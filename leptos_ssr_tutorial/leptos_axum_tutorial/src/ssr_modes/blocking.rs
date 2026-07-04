use leptos::prelude::*;
use leptos_meta::{Meta, Title};


use crate::ssr_modes::blog_elements;

#[component]
pub fn Blocking() -> impl IntoView {

    let post_data = Resource::new_blocking(
        || (),
        |_| blog_elements::blog_posts::get_posts()
    );

    let comments_data = Resource::new(
        || (),
        |_| blog_elements::blog_posts::get_comments()
    );

    
    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let data : Result<Vec<blog_elements::blog_posts::PostContent>, _> = post_data.await;
                view!{
                    <ul>
                        <For
                            each = move || data.clone().expect("Posts to be loaded") //.unwrap_or("Loading...")
                            key = |post| post.user.clone()
                            let(d)
                        >
                            <Title text=d.user.clone() />
                            <Meta name="description" content=d.post_data.clone() />
                            <li>
                                <blog_elements::blog_posts::BlogPost post=d.clone() />
                            </li>
                        </For>
                    </ul>
                }
            })}
        </Suspense>
        <Suspense fallback=|| "Loading comments...">
            {move || Suspend::new(async move {
                let comments : Result<Vec<blog_elements::blog_posts::Comment>, _> = comments_data.await;
                view! {
                    <ul>
                        <For
                            each = move || comments.clone().expect("Comments to be loaded") //.unwrap_or("Loading...")
                            key = |post| post.user.clone()
                            let(c)
                        >
                            <li>
                                <blog_elements::blog_posts::BlogPostComment comment=c.clone() />
                            </li>
                        </For>
                    </ul>
                }
            })}
        </Suspense>
    }
}