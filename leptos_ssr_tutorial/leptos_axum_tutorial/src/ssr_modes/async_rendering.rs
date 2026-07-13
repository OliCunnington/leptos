use leptos::prelude::*;
use crate::ssr_modes::blog_elements::blog_posts;

#[component]
pub fn AsynchronousRendering() -> impl IntoView  {
    // this count is our synchronous, local state
    let (count, set_count) = signal(0);

    // our resource
    let async_data = Resource::new(
        move || count.get(),
        // every time `count` changes, this will run
        |count| load_data(count) 
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
                    <li>
                        <p>"Placeholder"</p>
                    </li>
                </ul>
            </Suspense>
            <h2>"Comments"</h2>
            <Suspense fallback=|| view!{<p>"Loading..."</p>}>
                <ul>
                    <li>
                        <p>"Placeholder"</p>
                    </li>
                </ul>
            </Suspense>
        </div>
    }
}

async fn load_data(index: i32) -> Result<Vec<blog_posts::PostContent>, ServerFnError> {
    blog_posts::get_posts().await
}