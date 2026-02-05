use std::cell::RefCell;
use std::rc::Rc;
use leptos::prelude::*;
use leptos::html::Canvas;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::webgl::context::init_webgl2;
use crate::webgl::pipeline::AsciiPipeline;
use crate::webgl::animation::AnimationLoop;

#[component]
pub fn Hero() -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();

    let (cell_size, set_cell_size) = signal(8.0_f64);
    let (color_mix, set_color_mix) = signal(0.0_f64);

    // Shared state that bridges Leptos signals to the rAF loop
    let cell_size_rc: Rc<RefCell<f32>> = Rc::new(RefCell::new(8.0));
    let color_mix_rc: Rc<RefCell<f32>> = Rc::new(RefCell::new(0.0));

    // Keep animation loop alive across renders
    let anim_loop: Rc<RefCell<Option<AnimationLoop>>> = Rc::new(RefCell::new(None));

    // Sync Leptos signals to shared Rc<RefCell> values
    let cs_rc = cell_size_rc.clone();
    Effect::new(move |_| {
        *cs_rc.borrow_mut() = cell_size.get() as f32;
    });

    let cm_rc = color_mix_rc.clone();
    Effect::new(move |_| {
        *cm_rc.borrow_mut() = color_mix.get() as f32;
    });

    // Init WebGL + start animation
    let cs_init = cell_size_rc.clone();
    let cm_init = color_mix_rc.clone();
    let anim_init = anim_loop.clone();
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_el: &HtmlCanvasElement = &canvas;

            let w = canvas_el.client_width() as u32;
            let h = canvas_el.client_height() as u32;
            if w == 0 || h == 0 {
                return;
            }
            canvas_el.set_width(w);
            canvas_el.set_height(h);

            let gl = match init_webgl2(canvas_el) {
                Ok(gl) => gl,
                Err(e) => {
                    log::error!("WebGL2 init failed: {}", e);
                    return;
                }
            };

            let pipeline = match AsciiPipeline::new(gl, w, h) {
                Ok(p) => p,
                Err(e) => {
                    log::error!("Pipeline init failed: {}", e);
                    return;
                }
            };

            log::info!("ASCII pipeline started: {}x{}", w, h);
            let al = AnimationLoop::start(pipeline, cs_init.clone(), cm_init.clone());
            *anim_init.borrow_mut() = Some(al);
        }
    });

    // Handle window resize
    let anim_resize = anim_loop.clone();
    let cs_resize = cell_size_rc.clone();
    let cm_resize = color_mix_rc.clone();
    Effect::new(move |_| {
        let canvas_ref = canvas_ref.clone();
        let anim = anim_resize.clone();
        let cs = cs_resize.clone();
        let cm = cm_resize.clone();

        let closure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if let Some(canvas) = canvas_ref.get() {
                let canvas_el: &HtmlCanvasElement = &canvas;
                let w = canvas_el.client_width() as u32;
                let h = canvas_el.client_height() as u32;
                if w == 0 || h == 0 {
                    return;
                }
                canvas_el.set_width(w);
                canvas_el.set_height(h);

                let gl = match init_webgl2(canvas_el) {
                    Ok(gl) => gl,
                    Err(_) => return,
                };

                if let Ok(pipeline) = AsciiPipeline::new(gl, w, h) {
                    let al = AnimationLoop::start(pipeline, cs.clone(), cm.clone());
                    *anim.borrow_mut() = Some(al);
                }
            }
        });

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback(
                "resize",
                closure.as_ref().unchecked_ref(),
            );
        }
        closure.forget();
    });

    view! {
        <section class="hero">
            <div class="hero-canvas-wrap">
                <canvas node_ref=canvas_ref></canvas>
            </div>
            <div class="hero-overlay">
                <div class="container">
                    <h1>"Software Engineer"</h1>
                    <p class="tagline">"Building things with code, one character at a time."</p>
                </div>
            </div>
            <div class="hero-controls">
                <label>
                    "cell: "
                    <input
                        type="range"
                        min="4"
                        max="20"
                        step="1"
                        prop:value=move || cell_size.get().to_string()
                        on:input=move |ev| {
                            let val: f64 = event_target_value(&ev).parse().unwrap_or(8.0);
                            set_cell_size.set(val);
                        }
                    />
                </label>
                <label>
                    "color: "
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        prop:value=move || (color_mix.get() * 100.0).to_string()
                        on:input=move |ev| {
                            let val: f64 = event_target_value(&ev).parse().unwrap_or(0.0);
                            set_color_mix.set(val / 100.0);
                        }
                    />
                </label>
            </div>
        </section>
    }
}
