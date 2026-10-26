use leptos::html::Canvas;
use leptos::prelude::*;

use crate::render;

#[component]
pub fn App() -> impl IntoView {
    let canvas = NodeRef::<Canvas>::new();

    // Runs once the <canvas> exists in the DOM.
    // Next steps: create the World, hook up keyboard input, start a tick loop.
    Effect::new(move || {
        if let Some(canvas) = canvas.get() {
            render::draw(&canvas);
        }
    });

    view! {
        <main>
            <h1>"Snake"</h1>
            <canvas node_ref=canvas width="600" height="600"></canvas>
        </main>
    }
}
