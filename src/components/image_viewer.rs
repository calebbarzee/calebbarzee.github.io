use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    Blob, Event, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, Url,
};

use crate::webgl::animation::AnimationLoop;
use crate::webgl::context::{init_webgl2, sync_canvas_size};
use crate::webgl::pipeline::{Effect as EffectType, RenderFrame, Scene};

#[component]
pub fn ImageViewer() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let (selected_effect, set_selected_effect) = signal("ascii".to_string());
    let (image_loaded, set_image_loaded) = signal(false);

    // Shared state
    let img_el: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
    let anim_rc: Rc<RefCell<Option<AnimationLoop>>> = Rc::new(RefCell::new(None));

    // Effect control signals
    let (cell_size, set_cell_size) = signal(8.0_f64);
    let (color_mix, set_color_mix) = signal(0.0_f64);
    let (pixel_size, set_pixel_size) = signal(8.0_f64);
    let (threshold, set_threshold) = signal(0.2_f64);
    let (line_width, set_line_width) = signal(1.0_f64);

    // Start render loop
    let img_el2 = img_el.clone();
    let anim_rc2 = anim_rc.clone();
    let start_render = Rc::new(move || {
        let canvas: HtmlCanvasElement = canvas_ref.get().unwrap().into();
        let gl = init_webgl2(&canvas).unwrap();
        let (w, h) = sync_canvas_size(&canvas).unwrap_or((800, 600));

        let effect_name = selected_effect.get_untracked();
        let effect = match effect_name.as_str() {
            "ascii" => EffectType::Ascii,
            "passthrough" => EffectType::Passthrough,
            "pixelate" => EffectType::Pixelate,
            "edge_detect" => EffectType::EdgeDetect,
            _ => EffectType::Ascii,
        };

        let mut frame =
            RenderFrame::from_scene_effect(gl, w, h, Scene::Image, effect).unwrap();
        frame.create_scene_texture();

        // Upload initial image if available
        if let Some(ref img) = *img_el2.borrow() {
            frame.update_scene_texture_from_image(img);
        }

        let shared_frame = Rc::new(RefCell::new(frame));
        let img_for_loop = img_el2.clone();

        let anim = AnimationLoop::start_shared(shared_frame, move |f, _time| {
            // Re-upload every frame for animated GIFs (browser decodes frames)
            if let Some(ref img) = *img_for_loop.borrow() {
                f.update_scene_texture_from_image(img);
            }

            let w = f.width() as f32;
            let h = f.height() as f32;

            f.render_with_pre_scene(
                || {},
                |u| {
                    // Bind scene texture to unit 1 (unit 0 is FBO texture for effect)
                    f.bind_scene_texture(1);
                    u.set_i32("u_scene_tex", 1);
                    u.set_vec2("u_resolution", w, h);
                },
                |u| {
                    u.set_i32("u_scene", 0);
                    u.set_vec2("u_resolution", w, h);
                    u.set_f32("u_cell_size", cell_size.get_untracked() as f32);
                    u.set_f32("u_color_mix", color_mix.get_untracked() as f32);
                    u.set_f32("u_pixel_size", pixel_size.get_untracked() as f32);
                    u.set_f32("u_threshold", threshold.get_untracked() as f32);
                    u.set_f32("u_line_width", line_width.get_untracked() as f32);
                },
            );
        });

        *anim_rc2.borrow_mut() = Some(anim);
    });

    // File upload handler
    let img_el3 = img_el.clone();
    let start_render2 = start_render.clone();
    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let files = input.files().unwrap();
        if files.length() == 0 {
            return;
        }
        let file = files.get(0).unwrap();

        let blob: Blob = file.into();
        let url = Url::create_object_url_with_blob(&blob).unwrap();

        let img = HtmlImageElement::new().unwrap();
        let img_clone = img.clone();
        let img_store = img_el3.clone();
        let set_loaded = set_image_loaded;
        let start_fn = start_render2.clone();

        let onload = Closure::<dyn FnMut()>::new(move || {
            *img_store.borrow_mut() = Some(img_clone.clone());
            set_loaded.set(true);
            start_fn();
        });

        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        img.set_src(&url);
        onload.forget();
    };

    // Effect change handler
    let start_render3 = start_render.clone();
    let on_effect_change = move |ev: Event| {
        let select: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        set_selected_effect.set(select.value());
        if image_loaded.get_untracked() {
            start_render3();
        }
    };

    view! {
        <div class="image-viewer">
            <div class="upload-controls">
                <input
                    type="file"
                    accept="image/*,.gif"
                    on:change=on_file_change
                />
                <select on:change=on_effect_change>
                    <option value="ascii" selected>"ASCII"</option>
                    <option value="passthrough">"Passthrough"</option>
                    <option value="pixelate">"Pixelate"</option>
                    <option value="edge_detect">"Edge Detect"</option>
                </select>
            </div>
            <div class="viewer-canvas-wrap">
                {move || {
                    if !image_loaded.get() {
                        Some(view! {
                            <div class="upload-placeholder">
                                <p>"Upload an image or GIF to view"</p>
                            </div>
                        })
                    } else {
                        None
                    }
                }}
                <canvas
                    node_ref=canvas_ref
                    class="viewer-canvas"
                />
            </div>
            <div class="viewer-controls">
                {move || {
                    let eff = selected_effect.get();
                    match eff.as_str() {
                        "ascii" => view! {
                            <div class="control-group">
                                <label>
                                    "Cell Size: "
                                    <input
                                        type="range" min="4" max="20" step="1"
                                        prop:value=move || cell_size.get().to_string()
                                        on:input=move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(8.0);
                                            set_cell_size.set(v);
                                        }
                                    />
                                    <span>{move || format!("{:.0}", cell_size.get())}</span>
                                </label>
                                <label>
                                    "Color Mix: "
                                    <input
                                        type="range" min="0" max="100" step="1"
                                        prop:value=move || (color_mix.get() * 100.0).to_string()
                                        on:input=move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(0.0);
                                            set_color_mix.set(v / 100.0);
                                        }
                                    />
                                    <span>{move || format!("{:.0}%", color_mix.get() * 100.0)}</span>
                                </label>
                            </div>
                        }.into_any(),
                        "pixelate" => view! {
                            <div class="control-group">
                                <label>
                                    "Pixel Size: "
                                    <input
                                        type="range" min="2" max="32" step="1"
                                        prop:value=move || pixel_size.get().to_string()
                                        on:input=move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(8.0);
                                            set_pixel_size.set(v);
                                        }
                                    />
                                    <span>{move || format!("{:.0}", pixel_size.get())}</span>
                                </label>
                            </div>
                        }.into_any(),
                        "edge_detect" => view! {
                            <div class="control-group">
                                <label>
                                    "Threshold: "
                                    <input
                                        type="range" min="1" max="100" step="1"
                                        prop:value=move || (threshold.get() * 100.0).to_string()
                                        on:input=move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(20.0);
                                            set_threshold.set(v / 100.0);
                                        }
                                    />
                                    <span>{move || format!("{:.2}", threshold.get())}</span>
                                </label>
                                <label>
                                    "Line Width: "
                                    <input
                                        type="range" min="1" max="5" step="0.5"
                                        prop:value=move || line_width.get().to_string()
                                        on:input=move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(1.0);
                                            set_line_width.set(v);
                                        }
                                    />
                                    <span>{move || format!("{:.1}", line_width.get())}</span>
                                </label>
                            </div>
                        }.into_any(),
                        _ => view! {
                            <div class="control-group">
                                <p class="no-controls">"No controls for passthrough effect"</p>
                            </div>
                        }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}
