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
        <div style="font-family: -apple-system, BlinkMacSystemFont, 'Helvetica Neue', Arial, sans-serif; min-height: 100vh; background: linear-gradient(135deg, #ffecc5, #ffd4e2); padding: 48px;">
            <div style="max-width: 860px; margin: 0 auto; background: white; border-radius: 24px; padding: 36px; box-shadow: 0 20px 45px rgba(15, 23, 42, 0.18);">
                <div style="font-size: 14px; letter-spacing: 2px; text-transform: uppercase; color: #ce4573; font-weight: 700; margin-bottom: 12px;">
                    { "Module 10 - Asynchronous Programming" }
                </div>

                <h1 style="font-size: 42px; margin: 0 0 12px 0; color: #111827;">
                    { "YewChat WebClient" }
                </h1>

                <p style="font-size: 18px; line-height: 1.7; color: #374151; margin-bottom: 28px;">
                    { "Welcome to Kamali's ChatRoom App Thingy" }
                </p>

                <div style="display: grid; grid-template-columns: 1fr 2fr; gap: 18px; margin-bottom: 28px;">
                    <div style="border: 1px solid #e5e7eb; border-radius: 18px; padding: 20px; background: #f9fafb;">
                        <h3 style="margin: 0 0 12px 0; color: #111827;">{ "Connection" }</h3>
                        <p style="margin: 0 0 8px 0; color: #4b5563;">{ "WebClient: localhost:8000" }</p>
                        <p style="margin: 0; color: #4b5563;">{ "WebSocket: localhost:8080" }</p>
                    </div>

                    <div style="border: 1px solid #e5e7eb; border-radius: 18px; background: #f9fafb; overflow: hidden;">
                        <div style="background: linear-gradient(135deg, #ffecc5, #ffd4e2); color: #111827; padding: 14px 18px; font-weight: 700;">
                            { "Chat Room" }
                        </div>

                        <div style="padding: 18px; display: flex; flex-direction: column; gap: 12px;">
                            <div style="align-self: flex-start; max-width: 75%; background: #e0f2fe; color: #0f172a; padding: 12px 14px; border-radius: 14px;">
                                { "Server: Welcome to YewChat!" }
                            </div>

                            <div style="align-self: flex-end; max-width: 75%; background: #fee2e2; color: #0f172a; padding: 12px 14px; border-radius: 14px;">
                                { "Kamali: Hello from the webclient." }
                            </div>

                            <div style="align-self: flex-start; max-width: 75%; background: #e0f2fe; color: #0f172a; padding: 12px 14px; border-radius: 14px;">
                                { "Server: Message received through websocket flow." }
                            </div>

                            <div style="display: flex; gap: 10px; margin-top: 10px;">
                                <input
                                    value="Type a message..."
                                    readonly=true
                                    style="flex: 1; padding: 12px 14px; border: 1px solid #d1d5db; border-radius: 12px; color: #6b7280;"
                                />
                                <button style="padding: 12px 18px; border: none; border-radius: 12px; background: #ce4573; color: white; font-weight: 700;">
                                    { "Send" }
                                </button>
                            </div>
                        </div>
                    </div>
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