use leptos::prelude::*;
use crate::ssr_modes::blog_elements::blog_posts;

#[component]
pub fn AsynchronousRendering() -> impl IntoView  {
    // this count is our synchronous, local state
    let (count, set_count) = signal(0);

    // our resource
    let posts = Resource::new(
        move || count.get(),
        // every time `count` changes, this will run
        |count| load_posts(count as usize) 
    );

    let comments = Resource::new(
        move || count.get(),
        // every time `count` changes, this will run
        |count| load_comments(count as usize) 
    );

    view!{
        <div>
            <select
                on:change:target=move |ev| {
                set_count.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || count.get().to_string()
            >
                <option value="0">"0"</option>
                <option value="1">"1"</option>
                <option value="2">"2"</option>
                <option value="3">"3"</option>
            </select>
            <h2>"Posts"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    <For
                    each = move || posts.get().expect("Posts to be loaded").unwrap_or_default()
                    key = |post| post.user.clone()
                    let(child)
                    >
                        <li>
                            <blog_posts::BlogPost post=child />
                        </li>
                    </For>
                </ul>
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    <For
                    each = move || comments.get().expect("Comments to be loaded").unwrap_or_default()
                    key = |comment| comment.user.clone()
                    let(child)
                    >
                        <li>
                            <blog_posts::BlogPostComment comment=child />
                        </li>
                    </For>
                </ul>
            </Suspense>
        </div>
    }
}

async fn load_posts(index: usize) -> Result<Vec<blog_posts::PostContent>, ServerFnError> {
    match index {
        0 => blog_posts::get_posts().await,
        _ => {
            let r = blog_posts::get_post(index - 1).await;
            Ok(vec![r.unwrap()])
        }
    }
}

async fn load_comments(index: usize) -> Result<Vec<blog_posts::Comment>, ServerFnError> {
    match index {
        0 => blog_posts::get_comments().await,
        _ => {
            let r = blog_posts::get_comment(index - 1).await;
            Ok(vec![r.unwrap()])
        }
    }
}