use leptops::prelude::*;

[#component]
pub fn AsyncClosuresAndFutures() -> impl IntoView {
    view!{
        <table>
            <thead>
                <tr><th></th><th>Server-side rendering</th><th>During hydration (preparing for CSR)</th><th>Client-side rendering</th></tr>
            </thead>
            <tbody>
                <tr><td>Server Functions (Server)</td><td>Invoked as a normal asynchronous function, with the code defined in the body directly executed and the results returned to the caller. When executed inside a component, the full reactive context is accessible.</td><td>-</td><td>(Invoked by responding to the HTTP request made to the server function's standalone endpoint, where the response will be the serialized result. Since this endpoint is fully standalone, reactive context is typically not available.)</td></tr>
                <tr><td>Server Functions (Client)</td><td>-</td><td>Not executed. (When defined inside resource typed async closures.)</td><td>Makes an HTTP request to the server function's endpoint, the response body will be deserialized into the result. Take note that the code in the function body is never compiled into the client.</td></tr>
                <tr><td>Resources (Server)</td><td>Their async closures are executed in <em>parallel</em> when possible by the async executor; results are encoded into the response body inside <code>&lt;script&gt;</code> tags.</td><td>-</td><td>-</td></tr>
                <tr><td>Resources (Client)</td><td>-</td><td>Not executed; the <code>&lt;script&gt;</code> tags within the response body provides the results.</td><td>Their async closures are executed whenever required by the reactive system to return any new results.</td></tr>
                <tr><td>Local Resources (Client Only)</td><td>-</td><td>Not executed. (Will be executed once after hydration to provide the initial value.)</td><td>Their async closures are executed whenever required by the reactive system to return any new results.</td></tr>
                <tr><td>Suspend (Server)</td><td>These futures are executed in <em>parallel</em> when possible by the async executor.  The selected <code>SsrMode</code> can modify how and when: 1) these futures are polled, 2) the ordering of the stream of outputs.</td><td>-</td><td>-</td></tr>
                <tr><td>Suspend (Client)</td><td>-</td><td>These futures are fully executed once more in the client as part of hydration; returned <code>view! {}</code> will be used to hydrate the HTML.</td><td>These futures are executed whenever required by the reactive system; returned <code>view! {}</code>  will be rendered wherever they are defined.</td></tr>
            </tbody>
        </table>
    }
}