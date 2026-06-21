use leptos::prelude::*;

#[component]
pub fn AsynchronousRendering() -> impl IntoView  {
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