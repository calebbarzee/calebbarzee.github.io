use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::webgl::pipeline::AsciiPipeline;

/// Keeps the animation loop alive. Dropping this stops the loop.
pub struct AnimationLoop {
    // Prevent the closure from being dropped (which would kill the rAF loop)
    _keep_alive: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>,
}

impl AnimationLoop {
    pub fn start(
        pipeline: AsciiPipeline,
        cell_size: Rc<RefCell<f32>>,
        color_mix: Rc<RefCell<f32>>,
    ) -> Self {
        let pipeline = Rc::new(RefCell::new(pipeline));

        let cb: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let cb_clone = cb.clone();

        let closure = Closure::<dyn FnMut(f64)>::new(move |timestamp: f64| {
            let time = (timestamp / 1000.0) as f32;
            let cs = *cell_size.borrow();
            let cm = *color_mix.borrow();
            pipeline.borrow().render(time, cs, cm);

            // Schedule next frame
            if let Some(win) = window() {
                if let Some(ref c) = *cb_clone.borrow() {
                    let _ = win.request_animation_frame(c.as_ref().unchecked_ref());
                }
            }
        });

        // Kick off the first frame
        if let Some(win) = window() {
            let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
        }

        // Store the closure so it lives long enough
        *cb.borrow_mut() = Some(closure);

        AnimationLoop { _keep_alive: cb }
    }
}
