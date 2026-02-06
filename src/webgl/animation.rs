use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::webgl::pipeline::RenderFrame;

/// Keeps the animation loop alive. Dropping this stops the loop.
pub struct AnimationLoop {
    _keep_alive: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>,
}

impl AnimationLoop {
    /// Start a render loop.
    ///
    /// `frame_fn` is called each frame with (frame, time_seconds).
    /// It should call `frame.render(scene_setup, effect_setup)` with
    /// closures that set the appropriate uniforms.
    pub fn start<F>(frame: RenderFrame, frame_fn: F) -> Self
    where
        F: Fn(&RenderFrame, f32) + 'static,
    {
        let frame = Rc::new(RefCell::new(frame));

        let cb: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let cb_clone = cb.clone();

        let closure = Closure::<dyn FnMut(f64)>::new(move |timestamp: f64| {
            let time = (timestamp / 1000.0) as f32;
            let f = frame.borrow();
            frame_fn(&f, time);

            if let Some(win) = window() {
                if let Some(ref c) = *cb_clone.borrow() {
                    let _ = win.request_animation_frame(c.as_ref().unchecked_ref());
                }
            }
        });

        if let Some(win) = window() {
            let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
        }

        *cb.borrow_mut() = Some(closure);

        AnimationLoop { _keep_alive: cb }
    }

    /// Start a render loop with a shared (pre-wrapped) RenderFrame.
    /// This allows external code to mutate the frame (e.g. upload geometry)
    /// while the animation loop is running.
    pub fn start_shared<F>(frame: Rc<RefCell<RenderFrame>>, frame_fn: F) -> Self
    where
        F: Fn(&RenderFrame, f32) + 'static,
    {
        let cb: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let cb_clone = cb.clone();

        let closure = Closure::<dyn FnMut(f64)>::new(move |timestamp: f64| {
            let time = (timestamp / 1000.0) as f32;
            let f = frame.borrow();
            frame_fn(&f, time);

            if let Some(win) = window() {
                if let Some(ref c) = *cb_clone.borrow() {
                    let _ = win.request_animation_frame(c.as_ref().unchecked_ref());
                }
            }
        });

        if let Some(win) = window() {
            let _ = win.request_animation_frame(closure.as_ref().unchecked_ref());
        }

        *cb.borrow_mut() = Some(closure);

        AnimationLoop { _keep_alive: cb }
    }
}
