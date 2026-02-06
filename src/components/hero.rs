use leptos::prelude::*;

use crate::components::render_canvas::{EffectUniforms, RenderCanvas};
use crate::webgl::pipeline::{Effect as EffectType, Scene};

#[component]
pub fn Hero() -> impl IntoView {
    let (cell_size, set_cell_size) = signal(8.0_f64);
    let (color_mix, set_color_mix) = signal(0.0_f64);

    // Derive effect uniforms from signals
    let uniforms = Signal::derive(move || EffectUniforms {
        cell_size: cell_size.get() as f32,
        color_mix: color_mix.get() as f32,
        ..Default::default()
    });

    view! {
        <section class="hero">
            <RenderCanvas
                scene=Scene::Plasma
                effect=EffectType::Ascii
                uniforms=uniforms
                class="hero-canvas-wrap"
            />
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
