use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::render_canvas::{EffectUniforms, RenderCanvas};
use crate::webgl::pipeline::{Effect as EffectType, Scene};

/// Get all valid scene+effect combinations for the gallery
fn get_gallery_items() -> Vec<(Scene, EffectType)> {
    let mut items = Vec::new();
    for scene in Scene::all() {
        for effect in EffectType::all() {
            items.push((*scene, *effect));
        }
    }
    items
}

/// A single gallery item card
#[component]
fn GalleryCard(scene: Scene, effect: EffectType) -> impl IntoView {
    let href = format!("/gallery/{}/{}", scene.name(), effect.name());
    let title = format!("{} + {}", scene.display_name(), effect.display_name());

    // Default uniforms for preview
    let (uniforms, _) = signal(EffectUniforms::default());

    view! {
        <A href=href attr:class="gallery-card">
            <div class="gallery-card-preview">
                <RenderCanvas
                    scene=scene
                    effect=effect
                    uniforms=uniforms.into()
                    class="gallery-card-canvas"
                />
            </div>
            <div class="gallery-card-info">
                <h3>{title}</h3>
            </div>
        </A>
    }
}

/// Gallery grid showing all scene+effect combinations
#[component]
pub fn Gallery() -> impl IntoView {
    let items = get_gallery_items();

    view! {
        <section class="gallery">
            <div class="container">
                <h2>"$ ls gallery/"</h2>
                <div class="gallery-grid">
                    {items.into_iter().map(|(scene, effect)| {
                        view! { <GalleryCard scene=scene effect=effect /> }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

/// Parse scene from URL param
fn parse_scene(name: &str) -> Option<Scene> {
    Scene::all().iter().find(|s| s.name() == name).copied()
}

/// Parse effect from URL param
fn parse_effect(name: &str) -> Option<EffectType> {
    EffectType::all().iter().find(|e| e.name() == name).copied()
}

/// Gallery viewer for a specific scene+effect combination
#[component]
pub fn GalleryViewer() -> impl IntoView {
    let params = use_params_map();

    // Parse scene and effect from URL
    let scene_effect = move || {
        let p = params.read();
        let scene_name = p.get("scene").unwrap_or_default();
        let effect_name = p.get("effect").unwrap_or_default();

        match (parse_scene(&scene_name), parse_effect(&effect_name)) {
            (Some(s), Some(e)) => Some((s, e)),
            _ => None,
        }
    };

    view! {
        <div class="gallery-viewer">
            {move || {
                if let Some((scene, effect)) = scene_effect() {
                    view! {
                        <GalleryViewerContent scene=scene effect=effect />
                    }.into_any()
                } else {
                    view! {
                        <div class="gallery-viewer-error">
                            <p>"Invalid scene or effect"</p>
                            <A href="/gallery">"Back to gallery"</A>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// The actual viewer content with controls
#[component]
fn GalleryViewerContent(scene: Scene, effect: EffectType) -> impl IntoView {
    let title = format!("{} + {}", scene.display_name(), effect.display_name());

    // Effect-specific controls
    let (cell_size, set_cell_size) = signal(8.0_f64);
    let (color_mix, set_color_mix) = signal(0.0_f64);
    let (pixel_size, set_pixel_size) = signal(8.0_f64);
    let (threshold, set_threshold) = signal(0.2_f64);
    let (line_width, set_line_width) = signal(1.0_f64);

    let uniforms = Signal::derive(move || EffectUniforms {
        cell_size: cell_size.get() as f32,
        color_mix: color_mix.get() as f32,
        pixel_size: pixel_size.get() as f32,
        threshold: threshold.get() as f32,
        line_width: line_width.get() as f32,
    });

    view! {
        <div class="viewer-header">
            <A href="/gallery" attr:class="back-link">"< back to gallery"</A>
            <h1>{title}</h1>
        </div>
        <div class="viewer-canvas-wrap">
            <RenderCanvas
                scene=scene
                effect=effect
                uniforms=uniforms
                class="viewer-canvas"
            />
        </div>
        <div class="viewer-controls">
            {move || match effect {
                EffectType::Ascii => view! {
                    <div class="control-group">
                        <label>
                            "Cell Size: "
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
                            <span>{move || format!("{:.0}", cell_size.get())}</span>
                        </label>
                        <label>
                            "Color Mix: "
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
                            <span>{move || format!("{:.0}%", color_mix.get() * 100.0)}</span>
                        </label>
                    </div>
                }.into_any(),
                EffectType::Pixelate => view! {
                    <div class="control-group">
                        <label>
                            "Pixel Size: "
                            <input
                                type="range"
                                min="2"
                                max="32"
                                step="1"
                                prop:value=move || pixel_size.get().to_string()
                                on:input=move |ev| {
                                    let val: f64 = event_target_value(&ev).parse().unwrap_or(8.0);
                                    set_pixel_size.set(val);
                                }
                            />
                            <span>{move || format!("{:.0}", pixel_size.get())}</span>
                        </label>
                    </div>
                }.into_any(),
                EffectType::EdgeDetect => view! {
                    <div class="control-group">
                        <label>
                            "Threshold: "
                            <input
                                type="range"
                                min="1"
                                max="100"
                                step="1"
                                prop:value=move || (threshold.get() * 100.0).to_string()
                                on:input=move |ev| {
                                    let val: f64 = event_target_value(&ev).parse().unwrap_or(20.0);
                                    set_threshold.set(val / 100.0);
                                }
                            />
                            <span>{move || format!("{:.2}", threshold.get())}</span>
                        </label>
                        <label>
                            "Line Width: "
                            <input
                                type="range"
                                min="1"
                                max="5"
                                step="0.5"
                                prop:value=move || line_width.get().to_string()
                                on:input=move |ev| {
                                    let val: f64 = event_target_value(&ev).parse().unwrap_or(1.0);
                                    set_line_width.set(val);
                                }
                            />
                            <span>{move || format!("{:.1}", line_width.get())}</span>
                        </label>
                    </div>
                }.into_any(),
                EffectType::Passthrough => view! {
                    <div class="control-group">
                        <p class="no-controls">"No controls for passthrough effect"</p>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
