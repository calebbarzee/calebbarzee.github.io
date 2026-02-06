use web_sys::{
    WebGl2RenderingContext as GL, WebGlFramebuffer, WebGlTexture,
};

/// A render target consisting of a framebuffer and its color attachment texture.
/// Optionally includes a depth renderbuffer for 3D rendering.
pub struct RenderTarget {
    gl: GL,
    framebuffer: WebGlFramebuffer,
    texture: WebGlTexture,
    width: u32,
    height: u32,
    has_depth: bool,
}

impl RenderTarget {
    /// Create a new render target with the given dimensions.
    /// If `depth` is true, a depth renderbuffer is also attached.
    pub fn new(gl: &GL, width: u32, height: u32, depth: bool) -> Result<Self, String> {
        let texture = gl.create_texture().ok_or("Failed to create texture")?;
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
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
            Some(&texture),
            0,
        );

        if depth {
            let rb = gl.create_renderbuffer().ok_or("Failed to create renderbuffer")?;
            gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&rb));
            gl.renderbuffer_storage(
                GL::RENDERBUFFER,
                GL::DEPTH_COMPONENT24,
                width as i32,
                height as i32,
            );
            gl.framebuffer_renderbuffer(
                GL::FRAMEBUFFER,
                GL::DEPTH_ATTACHMENT,
                GL::RENDERBUFFER,
                Some(&rb),
            );
        }

        let status = gl.check_framebuffer_status(GL::FRAMEBUFFER);
        if status != GL::FRAMEBUFFER_COMPLETE {
            return Err(format!("Framebuffer incomplete: 0x{:x}", status));
        }

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.bind_texture(GL::TEXTURE_2D, None);

        Ok(Self {
            gl: gl.clone(),
            framebuffer,
            texture,
            width,
            height,
            has_depth: depth,
        })
    }

    /// Resize the render target. Recreates the texture (and depth buffer if present).
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.width = width;
        self.height = height;

        self.gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        self.gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
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
            .map_err(|e| format!("tex_image_2d resize failed: {:?}", e))?;

        if self.has_depth {
            self.gl
                .bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
            let rb = self
                .gl
                .create_renderbuffer()
                .ok_or("Failed to create renderbuffer on resize")?;
            self.gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&rb));
            self.gl.renderbuffer_storage(
                GL::RENDERBUFFER,
                GL::DEPTH_COMPONENT24,
                width as i32,
                height as i32,
            );
            self.gl.framebuffer_renderbuffer(
                GL::FRAMEBUFFER,
                GL::DEPTH_ATTACHMENT,
                GL::RENDERBUFFER,
                Some(&rb),
            );
            self.gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        }

        self.gl.bind_texture(GL::TEXTURE_2D, None);
        Ok(())
    }

    /// Bind this render target as the current framebuffer.
    pub fn bind(&self) {
        self.gl
            .bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
        self.gl
            .viewport(0, 0, self.width as i32, self.height as i32);
    }

    /// Bind the color texture to the given texture unit.
    pub fn bind_texture(&self, unit: u32) {
        self.gl.active_texture(GL::TEXTURE0 + unit);
        self.gl
            .bind_texture(GL::TEXTURE_2D, Some(&self.texture));
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
