pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use wasm_bindgen::{closure::Closure, JsCast};

    console_error_panic_hook::set_once();

    let callback = Closure::once(|| {
        leptos::logging::log!("starting delayed hydration");
        leptos::mount::hydrate_body(App);
    });

    web_sys::window()
        .expect("window should exist")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            2500,
        )
        .expect("setTimeout should register");
    callback.forget();
}
