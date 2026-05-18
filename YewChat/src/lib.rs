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
        <div style="font-family: Poppins, sans-serif; min-height: 100vh; background: linear-gradient(135deg, #05308e, #790606); padding: 48px;">
            <div style="max-width: 760px; margin: 0 auto; background: white; border-radius: 24px; padding: 36px; box-shadow: 0 20px 45px rgba(15, 23, 42, 0.12);">
                <div style="font-size: 14px; letter-spacing: 2px; text-transform: uppercase; color: #790606; font-weight: 700; margin-bottom: 12px;">
                    { "Module 10 - Asynchronous Programming" }
                </div>

                <h1 style="font-size: 42px; margin: 0 0 12px 0; color: #111827;">
                    { "YewChat WebClient" }
                </h1>

                <p style="font-size: 18px; line-height: 1.7; color: #374151; margin-bottom: 28px;">
                    { "A simple browser-based webclient built with Rust and Yew to demonstrate how asynchronous communication can be used in a websocket chat application." }
                </p>

                <div style="display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; margin-bottom: 28px;">
                    <div style="border: 1px solid #e5e7eb; border-radius: 18px; padding: 20px; background: #f9fafb;">
                        <h3 style="margin: 0 0 8px 0; color: #111827;">{ "WebClient" }</h3>
                        <p style="margin: 0; color: #4b5563;">{ "Running on localhost:8000" }</p>
                    </div>

                    <div style="border: 1px solid #e5e7eb; border-radius: 18px; padding: 20px; background: #f9fafb;">
                        <h3 style="margin: 0 0 8px 0; color: #111827;">{ "WebSocket Server" }</h3>
                        <p style="margin: 0; color: #4b5563;">{ "Expected on localhost:8080" }</p>
                    </div>
                </div>

                <div style="background: linear-gradient(135deg, #eff6ff, #fee2e2); border-left: 5px solid #05308e; padding: 18px 20px; border-radius: 14px; color: #05308e; line-height: 1.6;">
                    { "Creative note: This interface was customized to make the webclient clearer, more modern, and easier to understand as part of Experiment 3.2." }
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}