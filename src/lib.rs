pub mod game;
pub mod components;

use leptos::*;
use components::GameCanvas;
use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <GameCanvas />
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount_to_body(App);
}