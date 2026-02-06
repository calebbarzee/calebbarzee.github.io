use std::cell::RefCell;
use std::rc::Rc;
use leptos::prelude::*;
use leptos::html::Canvas;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::webgl::animation::AnimationLoop;
use crate::webgl::context::{init_webgl2, sync_canvas_size};
use crate::webgl::pipeline::{Effect as EffectType, RenderFrame, Scene};
use crate::webgl::uniform::UniformCache;

/// Props for effect-specific uniforms that get passed to the render loop.
#[derive(Clone)]
pub struct EffectUniforms {
    /// For ASCII effect
    pub cell_size: f32,
    pub color_mix: f32,
    /// For Pixelate effect
    pub pixel_size: f32,
    /// For EdgeDetect effect
    pub threshold: f32,
    pub line_width: f32,
}

impl Default for EffectUniforms {
    fn default() -> Self {
        Self {
            cell_size: 8.0,
            color_mix: 0.0,
            pixel_size: 8.0,
            threshold: 0.2,
            line_width: 1.0,
        }
    }
}

/// A reusable WebGL canvas component that renders a scene with an effect.
#[component]
pub fn RenderCanvas(
    /// The scene to render
    scene: Scene,
    /// The post-processing effect to apply
    effect: EffectType,
    /// Reactive effect uniforms (optional, use defaults if not provided)
    #[prop(optional)]
    uniforms: Option<Signal<EffectUniforms>>,
    /// CSS class for the canvas wrapper
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();

    // Use provided uniforms or create defaults
    let uniforms = uniforms.unwrap_or_else(|| {
        let (sig, _) = signal(EffectUniforms::default());
        sig.into()
    });

    // Bridge to animation loop
    let uniforms_rc: Rc<RefCell<EffectUniforms>> = Rc::new(RefCell::new(EffectUniforms::default()));

    // Sync signal to Rc<RefCell>
    let u_rc = uniforms_rc.clone();
    leptos::prelude::Effect::new(move |_| {
        *u_rc.borrow_mut() = uniforms.get();
    });

    // Keep animation loop alive
    let anim_loop: Rc<RefCell<Option<AnimationLoop>>> = Rc::new(RefCell::new(None));

    // Initialize WebGL and start animation
    let u_init = uniforms_rc.clone();
    let anim_init = anim_loop.clone();
    leptos::prelude::Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_el: &HtmlCanvasElement = &canvas;

            let (w, h) = match sync_canvas_size(canvas_el) {
                Some(size) => size,
                None => return,
            };

            let gl = match init_webgl2(canvas_el) {
                Ok(gl) => gl,
                Err(e) => {
                    log::error!("WebGL2 init failed: {}", e);
                    return;
                }
            };

            let frame = match RenderFrame::from_scene_effect(gl, w, h, scene, effect) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("RenderFrame init failed: {}", e);
                    return;
                }
            };

            let u = u_init.clone();
            let eff = effect;
            let al = AnimationLoop::start(frame, move |f, time| {
                let eu = u.borrow();
                let width = f.width() as f32;
                let height = f.height() as f32;

                f.render(
                    |scene_u| {
                        scene_u.set_f32("u_time", time);
                        scene_u.set_vec2("u_resolution", width, height);
                    },
                    |effect_u| {
                        effect_u.set_i32("u_scene", 0);
                        effect_u.set_vec2("u_resolution", width, height);
                        set_effect_uniforms(eff, effect_u, &eu);
                    },
                );
            });
            *anim_init.borrow_mut() = Some(al);
        }
    });

    // Handle window resize
    let anim_resize = anim_loop.clone();
    let u_resize = uniforms_rc.clone();
    leptos::prelude::Effect::new(move |_| {
        let canvas_ref = canvas_ref.clone();
        let anim = anim_resize.clone();
        let u = u_resize.clone();

        let closure = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
            if let Some(canvas) = canvas_ref.get() {
                let canvas_el: &HtmlCanvasElement = &canvas;
                let (w, h) = match sync_canvas_size(canvas_el) {
                    Some(size) => size,
                    None => return,
                };

                let gl = match init_webgl2(canvas_el) {
                    Ok(gl) => gl,
                    Err(_) => return,
                };

                if let Ok(frame) = RenderFrame::from_scene_effect(gl, w, h, scene, effect) {
                    let u2 = u.clone();
                    let eff = effect;
                    let al = AnimationLoop::start(frame, move |f, time| {
                        let eu = u2.borrow();
                        let width = f.width() as f32;
                        let height = f.height() as f32;

                        f.render(
                            |scene_u| {
                                scene_u.set_f32("u_time", time);
                                scene_u.set_vec2("u_resolution", width, height);
                            },
                            |effect_u| {
                                effect_u.set_i32("u_scene", 0);
                                effect_u.set_vec2("u_resolution", width, height);
                                set_effect_uniforms(eff, effect_u, &eu);
                            },
                        );
                    });
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

    let wrapper_class = class.unwrap_or("render-canvas-wrap");

    view! {
        <div class=wrapper_class>
            <canvas node_ref=canvas_ref></canvas>
        </div>
    }
}

/// Set effect-specific uniforms based on the effect type.
fn set_effect_uniforms(effect: EffectType, u: &UniformCache, eu: &EffectUniforms) {
    match effect {
        EffectType::Ascii => {
            u.set_f32("u_cell_size", eu.cell_size);
            u.set_f32("u_color_mix", eu.color_mix);
        }
        EffectType::Passthrough => {
            // No extra uniforms
        }
        EffectType::Pixelate => {
            u.set_f32("u_pixel_size", eu.pixel_size);
        }
        EffectType::EdgeDetect => {
            u.set_f32("u_threshold", eu.threshold);
            u.set_f32("u_line_width", eu.line_width);
        }
    }
}
