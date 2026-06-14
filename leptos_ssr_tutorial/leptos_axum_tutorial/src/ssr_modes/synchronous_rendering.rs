use leptos::prelude::*;

#[component]
pub fn SynchronousRendering() {
    view!{
        <div>
            <Suspense fallback=|| (view!{<p>"Loading..."</p>})>
                // local resources? eh?
            </Suspense>
        </div>
    }
}