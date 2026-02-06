use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub fn init_webgl2(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, String> {
    let gl = canvas
        .get_context("webgl2")
        .map_err(|e| format!("get_context failed: {:?}", e))?
        .ok_or("WebGL2 not supported")?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| "Failed to cast to WebGl2RenderingContext")?;

    gl.clear_color(0.106, 0.055, 0.090, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    Ok(gl)
}

/// Sync the canvas drawing buffer size to its CSS layout size.
/// Returns (width, height) or None if the canvas has zero size.
pub fn sync_canvas_size(canvas: &HtmlCanvasElement) -> Option<(u32, u32)> {
    let w = canvas.client_width() as u32;
    let h = canvas.client_height() as u32;
    if w == 0 || h == 0 {
        return None;
    }
    canvas.set_width(w);
    canvas.set_height(h);
    Some((w, h))
}
