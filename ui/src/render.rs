use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .flatten()?
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
}

/// Placeholder: clears the canvas. Will eventually take `&World` and draw it.
pub fn draw(canvas: &HtmlCanvasElement) {
    let Some(ctx) = context(canvas) else { return };
    ctx.set_fill_style_str("black");
    ctx.fill_rect(0.0, 0.0, f64::from(canvas.width()), f64::from(canvas.height()));
}
