use wasm_bindgen::prelude::*;
use web_sys::*;

mod renderer;
use renderer::Renderer;

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = get_canvas_by_id(canvas_id)?;
    let gl = get_webgl_context(&canvas)?;

    let renderer = Renderer::new(gl)?;
    renderer.draw_triangle();

    Ok(())
}

fn get_canvas_by_id(canvas_id: &str) -> Result<HtmlCanvasElement, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("missing document"))?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str(&format!("missing canvas element id='{canvas_id}'")))?;
    element
        .dyn_into::<HtmlCanvasElement>()
        .map_err(JsValue::from)
}

fn get_webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGlRenderingContext, JsValue> {
    let ctx = canvas
        .get_context("webgl")?
        .ok_or_else(|| JsValue::from_str("WebGL context unavailable"))?;
    ctx.dyn_into::<WebGlRenderingContext>()
        .map_err(JsValue::from)
}
