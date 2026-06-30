use leptos::prelude::*;
use leptos_meta::{Meta, Title};


use crate::ssr_modes::blog_elements;

#[component]
pub fn BlogPost() -> impl IntoView {
    let (post_count, set_post_count) = signal(0);
    let (comment_count, set_comment_count) = signal(0);
    let post_data = Resource::new_blocking(
        move || post_count.get(),
        |_| blog_elements::blog_posts::get_posts()
    );
    let comments_data = Resource::new(
        move || comment_count.get(),
        |_| blog_elements::blog_posts::get_comments()
    );
    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let data : Result<Vec<blog_elements::blog_posts::PostContent>, _> = post_data.await;
                view!{
                    <ul>
                        <For
                            each = move || data.as_ref().expect("Posts to be loaded") //.unwrap_or("Loading...")
                            key = |post| post.user.clone()
                            let(d)
                        >
                            <Title text=d.user.clone() />
                            <Meta name="description" content=d.postData.clone() />
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
                            each = move || comments.as_ref().expect("Comments to be loaded") //.unwrap_or("Loading...")
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