use web_sys::{
    WebGl2RenderingContext as GL, WebGlFramebuffer, WebGlProgram, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

use crate::webgl::shader::link_program;

const SCENE_VERT: &str = include_str!("../shaders/scene.vert");
const SCENE_FRAG: &str = include_str!("../shaders/scene.frag");
const ASCII_VERT: &str = include_str!("../shaders/ascii.vert");
const ASCII_FRAG: &str = include_str!("../shaders/ascii.frag");

pub struct AsciiPipeline {
    gl: GL,
    scene_program: WebGlProgram,
    ascii_program: WebGlProgram,
    framebuffer: WebGlFramebuffer,
    fb_texture: WebGlTexture,
    empty_vao: WebGlVertexArrayObject,
    // Scene uniforms
    u_time: Option<WebGlUniformLocation>,
    u_resolution_scene: Option<WebGlUniformLocation>,
    // ASCII uniforms
    u_scene_tex: Option<WebGlUniformLocation>,
    u_resolution_ascii: Option<WebGlUniformLocation>,
    u_cell_size: Option<WebGlUniformLocation>,
    u_color_mix: Option<WebGlUniformLocation>,
    width: u32,
    height: u32,
}

impl AsciiPipeline {
    pub fn new(gl: GL, width: u32, height: u32) -> Result<Self, String> {
        let scene_program = link_program(&gl, SCENE_VERT, SCENE_FRAG)?;
        let ascii_program = link_program(&gl, ASCII_VERT, ASCII_FRAG)?;

        // Get uniform locations
        let u_time = gl.get_uniform_location(&scene_program, "u_time");
        let u_resolution_scene = gl.get_uniform_location(&scene_program, "u_resolution");
        let u_scene_tex = gl.get_uniform_location(&ascii_program, "u_scene");
        let u_resolution_ascii = gl.get_uniform_location(&ascii_program, "u_resolution");
        let u_cell_size = gl.get_uniform_location(&ascii_program, "u_cell_size");
        let u_color_mix = gl.get_uniform_location(&ascii_program, "u_color_mix");

        // Create empty VAO (needed for fullscreen triangle with no vertex buffers)
        let empty_vao = gl.create_vertex_array().ok_or("Failed to create VAO")?;

        // Create framebuffer + texture for scene pass
        let fb_texture = gl.create_texture().ok_or("Failed to create texture")?;
        gl.bind_texture(GL::TEXTURE_2D, Some(&fb_texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            width as i32,
            height as i32,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            None,
        )
        .map_err(|e| format!("tex_image_2d failed: {:?}", e))?;
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        let framebuffer = gl.create_framebuffer().ok_or("Failed to create framebuffer")?;
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            GL::COLOR_ATTACHMENT0,
            GL::TEXTURE_2D,
            Some(&fb_texture),
            0,
        );

        let status = gl.check_framebuffer_status(GL::FRAMEBUFFER);
        if status != GL::FRAMEBUFFER_COMPLETE {
            return Err(format!("Framebuffer incomplete: 0x{:x}", status));
        }

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(Self {
            gl,
            scene_program,
            ascii_program,
            framebuffer,
            fb_texture,
            empty_vao,
            u_time,
            u_resolution_scene,
            u_scene_tex,
            u_resolution_ascii,
            u_cell_size,
            u_color_mix,
            width,
            height,
        })
    }

    pub fn render(&self, time: f32, cell_size: f32, color_mix: f32) {
        let gl = &self.gl;

        gl.bind_vertex_array(Some(&self.empty_vao));

        // ── Pass 1: Render scene to framebuffer ─────────────
        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
        gl.viewport(0, 0, self.width as i32, self.height as i32);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&self.scene_program));
        gl.uniform1f(self.u_time.as_ref(), time);
        gl.uniform2f(
            self.u_resolution_scene.as_ref(),
            self.width as f32,
            self.height as f32,
        );
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        // ── Pass 2: ASCII post-processing to screen ─────────
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.viewport(0, 0, self.width as i32, self.height as i32);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&self.ascii_program));

        // Bind scene texture to unit 0
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.fb_texture));
        gl.uniform1i(self.u_scene_tex.as_ref(), 0);

        gl.uniform2f(
            self.u_resolution_ascii.as_ref(),
            self.width as f32,
            self.height as f32,
        );
        gl.uniform1f(self.u_cell_size.as_ref(), cell_size);
        gl.uniform1f(self.u_color_mix.as_ref(), color_mix);

        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        gl.bind_vertex_array(None);
    }
}
