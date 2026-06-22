use leptos::prelude::*;

#[component]
pub fn InOrderStreaming() -> impl IntoView {
    view!{
        <div>
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