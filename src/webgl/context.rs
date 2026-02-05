use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub fn init_webgl2(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, String> {
    let gl = canvas
        .get_context("webgl2")
        .map_err(|e| format!("get_context failed: {:?}", e))?
        .ok_or("WebGL2 not supported")?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| "Failed to cast to WebGl2RenderingContext")?;

    gl.clear_color(0.04, 0.04, 0.04, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    Ok(gl)
}
