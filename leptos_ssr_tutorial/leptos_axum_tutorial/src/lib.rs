pub mod app;
pub mod ssr_modes;
pub mod server_functions;
pub mod extractors;
pub mod action_forms;
pub mod async_reference;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
