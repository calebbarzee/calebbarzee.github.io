use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    Event, FileReader, HtmlCanvasElement, HtmlInputElement, MouseEvent, WheelEvent,
};

use crate::stl::parse_stl;
use crate::webgl::animation::AnimationLoop;
use crate::webgl::camera::OrbitCamera;
use crate::webgl::context::{init_webgl2, sync_canvas_size};
use crate::webgl::pipeline::{Effect as EffectType, RenderFrame, Scene};

#[component]
pub fn StlViewer() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let (selected_effect, set_selected_effect) = signal("ascii".to_string());
    let (model_loaded, set_model_loaded) = signal(false);

    // Shared state
    let frame_rc: Rc<RefCell<Option<Rc<RefCell<RenderFrame>>>>> = Rc::new(RefCell::new(None));
    let camera_rc: Rc<RefCell<OrbitCamera>> = Rc::new(RefCell::new(OrbitCamera::default()));
    let mesh_data: Rc<RefCell<Option<MeshData>>> = Rc::new(RefCell::new(None));
    let anim_rc: Rc<RefCell<Option<AnimationLoop>>> = Rc::new(RefCell::new(None));

    // Effect control signals
    let (cell_size, set_cell_size) = signal(8.0_f64);
    let (color_mix, set_color_mix) = signal(0.0_f64);
    let (pixel_size, set_pixel_size) = signal(8.0_f64);
    let (threshold, set_threshold) = signal(0.2_f64);
    let (line_width, set_line_width) = signal(1.0_f64);

    // Start the render loop with current effect
    let frame_rc2 = frame_rc.clone();
    let camera_rc2 = camera_rc.clone();
    let mesh_data2 = mesh_data.clone();
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

        let mut frame = RenderFrame::from_scene_effect(gl, w, h, Scene::Stl, effect).unwrap();

        // Re-upload geometry if we have it
        if let Some(ref md) = *mesh_data2.borrow() {
            frame.upload_geometry(&md.positions, &md.normals);
        }

        let shared_frame = Rc::new(RefCell::new(frame));
        *frame_rc2.borrow_mut() = Some(shared_frame.clone());

        let cam = camera_rc2.clone();
        let md = mesh_data2.clone();

        let anim = AnimationLoop::start_shared(shared_frame, move |f, _time| {
            let cam_ref = cam.borrow();
            let md_ref = md.borrow();
            let model_mat = if let Some(ref mesh) = *md_ref {
                mesh.model_matrix()
            } else {
                glam::Mat4::IDENTITY
            };

            let w = f.width() as f32;
            let h = f.height() as f32;
            let aspect = if h > 0.0 { w / h } else { 1.0 };
            let view = cam_ref.view_matrix();
            let proj = cam_ref.projection_matrix(aspect);
            let eye = cam_ref.eye_position();

            f.render(
                |u| {
                    u.set_mat4("u_model", &model_mat.to_cols_array());
                    u.set_mat4("u_view", &view.to_cols_array());
                    u.set_mat4("u_projection", &proj.to_cols_array());
                    u.set_vec3("u_light_dir", 0.5, 0.8, 0.6);
                    u.set_vec3("u_light_color", 1.0, 1.0, 1.0);
                    u.set_vec3("u_ambient", 0.15, 0.15, 0.15);
                    u.set_vec3("u_object_color", 0.7, 0.75, 0.8);
                    u.set_vec3("u_eye_pos", eye.x, eye.y, eye.z);
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
    let frame_rc3 = frame_rc.clone();
    let mesh_data3 = mesh_data.clone();
    let start_render2 = start_render.clone();
    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let files = input.files().unwrap();
        if files.length() == 0 {
            return;
        }
        let file = files.get(0).unwrap();
        let reader = FileReader::new().unwrap();

        let frame_clone = frame_rc3.clone();
        let mesh_clone = mesh_data3.clone();
        let set_loaded = set_model_loaded;
        let start_fn = start_render2.clone();

        let onload = Closure::<dyn FnMut(Event)>::new(move |ev: Event| {
            let reader: FileReader = ev.target().unwrap().dyn_into().unwrap();
            let result = reader.result().unwrap();
            let array = js_sys::Uint8Array::new(&result);
            let data = array.to_vec();

            match parse_stl(&data) {
                Ok(mesh) => {
                    let md = MeshData::from_stl(&mesh);

                    // Upload to existing frame if present
                    if let Some(ref shared) = *frame_clone.borrow() {
                        shared
                            .borrow_mut()
                            .upload_geometry(&md.positions, &md.normals);
                    }

                    *mesh_clone.borrow_mut() = Some(md);
                    set_loaded.set(true);

                    // If no animation running yet, start it
                    start_fn();
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("STL parse error: {}", e).into());
                }
            }
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.read_as_array_buffer(&file).unwrap();
        onload.forget();
    };

    // Effect change handler
    let start_render3 = start_render.clone();
    let on_effect_change = move |ev: Event| {
        let select: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        set_selected_effect.set(select.value());
        if model_loaded.get_untracked() {
            start_render3();
        }
    };

    // Mouse handlers for orbit camera
    let cam_down = camera_rc.clone();
    let cam_move = camera_rc.clone();
    let cam_up = camera_rc.clone();
    let cam_wheel = camera_rc.clone();

    let on_mousedown = move |ev: MouseEvent| {
        ev.prevent_default();
        cam_down
            .borrow_mut()
            .on_mouse_down(ev.client_x() as f32, ev.client_y() as f32);
    };

    let on_mousemove = move |ev: MouseEvent| {
        cam_move
            .borrow_mut()
            .on_mouse_move(ev.client_x() as f32, ev.client_y() as f32);
    };

    let on_mouseup = move |_ev: MouseEvent| {
        cam_up.borrow_mut().on_mouse_up();
    };

    let on_wheel = move |ev: WheelEvent| {
        ev.prevent_default();
        cam_wheel.borrow_mut().on_wheel(ev.delta_y() as f32);
    };

    // Mount effect: start render if model already loaded
    let start_render4 = start_render.clone();
    Effect::new(move || {
        // Initialize canvas on mount
        if canvas_ref.get().is_some() && model_loaded.get() {
            start_render4();
        }
    });

    view! {
        <div class="stl-viewer">
            <div class="upload-controls">
                <input
                    type="file"
                    accept=".stl"
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
                    if !model_loaded.get() {
                        Some(view! {
                            <div class="upload-placeholder">
                                <p>"Upload an .stl file to view"</p>
                            </div>
                        })
                    } else {
                        None
                    }
                }}
                <canvas
                    node_ref=canvas_ref
                    class="viewer-canvas"
                    on:mousedown=on_mousedown
                    on:mousemove=on_mousemove
                    on:mouseup=on_mouseup
                    on:wheel=on_wheel
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

/// Cached mesh data ready for GPU upload.
struct MeshData {
    positions: Vec<f32>,
    normals: Vec<f32>,
    center: [f32; 3],
    extent: f32,
}

impl MeshData {
    fn from_stl(mesh: &crate::stl::StlMesh) -> Self {
        Self {
            positions: mesh.positions.clone(),
            normals: mesh.normals.clone(),
            center: mesh.center(),
            extent: mesh.extent(),
        }
    }

    /// Compute a model matrix that centers and normalizes the mesh to a unit cube.
    fn model_matrix(&self) -> glam::Mat4 {
        let scale = if self.extent > 0.0 {
            2.0 / self.extent
        } else {
            1.0
        };
        let c = self.center;
        glam::Mat4::from_scale(glam::Vec3::splat(scale))
            * glam::Mat4::from_translation(glam::Vec3::new(-c[0], -c[1], -c[2]))
    }
}
