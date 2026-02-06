use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlUniformLocation};

/// Caches uniform locations for a shader program and provides typed setters.
pub struct UniformCache {
    gl: GL,
    locations: HashMap<&'static str, Option<WebGlUniformLocation>>,
}

impl UniformCache {
    /// Build a uniform cache for the given program and uniform names.
    pub fn new(gl: &GL, program: &WebGlProgram, names: &[&'static str]) -> Self {
        let mut locations = HashMap::with_capacity(names.len());
        for &name in names {
            locations.insert(name, gl.get_uniform_location(program, name));
        }
        Self {
            gl: gl.clone(),
            locations,
        }
    }

    fn loc(&self, name: &str) -> Option<&WebGlUniformLocation> {
        self.locations.get(name).and_then(|opt| opt.as_ref())
    }

    pub fn set_f32(&self, name: &str, v: f32) {
        self.gl.uniform1f(self.loc(name), v);
    }

    pub fn set_i32(&self, name: &str, v: i32) {
        self.gl.uniform1i(self.loc(name), v);
    }

    pub fn set_vec2(&self, name: &str, x: f32, y: f32) {
        self.gl.uniform2f(self.loc(name), x, y);
    }

    pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        self.gl.uniform3f(self.loc(name), x, y, z);
    }

    pub fn set_mat4(&self, name: &str, m: &[f32; 16]) {
        self.gl
            .uniform_matrix4fv_with_f32_array(self.loc(name), false, m);
    }
}
