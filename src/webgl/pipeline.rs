use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlVertexArrayObject};

use crate::webgl::framebuffer::RenderTarget;
use crate::webgl::shader::link_program;
use crate::webgl::uniform::UniformCache;

// ── Shader sources ─────────────────────────────────────────
const FULLSCREEN_TRIANGLE_VERT: &str = include_str!("../shaders/fullscreen_triangle.vert");

// Scene shaders
const PLASMA_FRAG: &str = include_str!("../shaders/plasma.frag");

// Effect shaders
const ASCII_FRAG: &str = include_str!("../shaders/ascii.frag");
const PASSTHROUGH_FRAG: &str = include_str!("../shaders/passthrough.frag");
const PIXELATE_FRAG: &str = include_str!("../shaders/pixelate.frag");
const EDGE_DETECT_FRAG: &str = include_str!("../shaders/edge_detect.frag");

// ── Scene and Effect definitions ───────────────────────────

/// Available scene types (what gets rendered to the framebuffer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scene {
    Plasma,
}

impl Scene {
    pub fn all() -> &'static [Scene] {
        &[Scene::Plasma]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Scene::Plasma => "plasma",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Scene::Plasma => "Plasma",
        }
    }

    fn frag_source(&self) -> &'static str {
        match self {
            Scene::Plasma => PLASMA_FRAG,
        }
    }

    fn uniforms(&self) -> &'static [&'static str] {
        match self {
            Scene::Plasma => &["u_time", "u_resolution"],
        }
    }
}

/// Available post-processing effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Ascii,
    Passthrough,
    Pixelate,
    EdgeDetect,
}

impl Effect {
    pub fn all() -> &'static [Effect] {
        &[Effect::Ascii, Effect::Passthrough, Effect::Pixelate, Effect::EdgeDetect]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Effect::Ascii => "ascii",
            Effect::Passthrough => "passthrough",
            Effect::Pixelate => "pixelate",
            Effect::EdgeDetect => "edge_detect",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Effect::Ascii => "ASCII",
            Effect::Passthrough => "Passthrough",
            Effect::Pixelate => "Pixelate",
            Effect::EdgeDetect => "Edge Detect",
        }
    }

    fn frag_source(&self) -> &'static str {
        match self {
            Effect::Ascii => ASCII_FRAG,
            Effect::Passthrough => PASSTHROUGH_FRAG,
            Effect::Pixelate => PIXELATE_FRAG,
            Effect::EdgeDetect => EDGE_DETECT_FRAG,
        }
    }

    fn uniforms(&self) -> &'static [&'static str] {
        match self {
            Effect::Ascii => &["u_scene", "u_resolution", "u_cell_size", "u_color_mix"],
            Effect::Passthrough => &["u_scene", "u_resolution"],
            Effect::Pixelate => &["u_scene", "u_resolution", "u_pixel_size"],
            Effect::EdgeDetect => &["u_scene", "u_resolution", "u_threshold", "u_line_width"],
        }
    }
}

// ── FrameConfig ────────────────────────────────────────────

/// Configuration that defines what a frame renders and how.
pub struct FrameConfig {
    pub scene_vert: &'static str,
    pub scene_frag: &'static str,
    pub effect_frag: &'static str,
    pub depth_test: bool,
    pub scene_vertex_count: i32,
    pub scene_uniforms: &'static [&'static str],
    pub effect_uniforms: &'static [&'static str],
}

// ── RenderFrame ────────────────────────────────────────────

/// A fully initialized, ready-to-render frame.
pub struct RenderFrame {
    gl: GL,
    scene_program: WebGlProgram,
    effect_program: WebGlProgram,
    render_target: RenderTarget,
    scene_vao: WebGlVertexArrayObject,
    effect_vao: WebGlVertexArrayObject,
    scene_uniforms: UniformCache,
    effect_uniforms: UniformCache,
    depth_test: bool,
    scene_vertex_count: i32,
    width: u32,
    height: u32,
}

impl RenderFrame {
    /// Create a new RenderFrame from a FrameConfig.
    pub fn new(gl: GL, width: u32, height: u32, config: FrameConfig) -> Result<Self, String> {
        let scene_program = link_program(&gl, config.scene_vert, config.scene_frag)?;
        let effect_program =
            link_program(&gl, FULLSCREEN_TRIANGLE_VERT, config.effect_frag)?;

        let scene_uniforms =
            UniformCache::new(&gl, &scene_program, config.scene_uniforms);
        let effect_uniforms =
            UniformCache::new(&gl, &effect_program, config.effect_uniforms);

        let scene_vao = gl
            .create_vertex_array()
            .ok_or("Failed to create scene VAO")?;
        let effect_vao = gl
            .create_vertex_array()
            .ok_or("Failed to create effect VAO")?;

        let render_target = RenderTarget::new(&gl, width, height, config.depth_test)?;

        Ok(Self {
            gl,
            scene_program,
            effect_program,
            render_target,
            scene_vao,
            effect_vao,
            scene_uniforms,
            effect_uniforms,
            depth_test: config.depth_test,
            scene_vertex_count: config.scene_vertex_count,
            width,
            height,
        })
    }

    /// Create a RenderFrame from Scene and Effect enums.
    pub fn from_scene_effect(
        gl: GL,
        width: u32,
        height: u32,
        scene: Scene,
        effect: Effect,
    ) -> Result<Self, String> {
        Self::new(
            gl,
            width,
            height,
            FrameConfig {
                scene_vert: FULLSCREEN_TRIANGLE_VERT,
                scene_frag: scene.frag_source(),
                effect_frag: effect.frag_source(),
                depth_test: false,
                scene_vertex_count: 3,
                scene_uniforms: scene.uniforms(),
                effect_uniforms: effect.uniforms(),
            },
        )
    }

    /// Factory: plasma scene with ASCII effect (convenience method).
    pub fn plasma_ascii(gl: GL, width: u32, height: u32) -> Result<Self, String> {
        Self::from_scene_effect(gl, width, height, Scene::Plasma, Effect::Ascii)
    }

    /// Resize the frame without recompiling shaders.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.width = width;
        self.height = height;
        self.render_target.resize(width, height)
    }

    /// Render one frame.
    pub fn render<S, E>(&self, scene_setup: S, effect_setup: E)
    where
        S: FnOnce(&UniformCache),
        E: FnOnce(&UniformCache),
    {
        let gl = &self.gl;

        // ── Pass 1: Render scene to framebuffer ─────────────
        self.render_target.bind();
        let clear_bits = if self.depth_test {
            gl.enable(GL::DEPTH_TEST);
            GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT
        } else {
            gl.disable(GL::DEPTH_TEST);
            GL::COLOR_BUFFER_BIT
        };
        gl.clear(clear_bits);

        gl.use_program(Some(&self.scene_program));
        scene_setup(&self.scene_uniforms);
        gl.bind_vertex_array(Some(&self.scene_vao));
        gl.draw_arrays(GL::TRIANGLES, 0, self.scene_vertex_count);

        // ── Pass 2: Post-processing effect to screen ────────
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.viewport(0, 0, self.width as i32, self.height as i32);
        gl.disable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&self.effect_program));
        self.render_target.bind_texture(0);
        effect_setup(&self.effect_uniforms);
        gl.bind_vertex_array(Some(&self.effect_vao));
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        gl.bind_vertex_array(None);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
