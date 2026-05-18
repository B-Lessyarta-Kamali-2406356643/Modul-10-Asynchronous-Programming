#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(Main)]
fn main_component() -> Html {
    html! {
        <div style="font-family: Arial, sans-serif; padding: 32px;">
            <h1>{ "YewChat" }</h1>
            <p>{ "Original YewChat webclient is running." }</p>
            <p>{ "WebSocket server should run on localhost:8080." }</p>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}