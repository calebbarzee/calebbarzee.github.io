use glam::{Mat4, Vec3};

/// An orbit camera that rotates around a target point.
pub struct OrbitCamera {
    pub azimuth: f32,
    pub elevation: f32,
    pub distance: f32,
    pub target: Vec3,
    pub dragging: bool,
    pub last_mouse: (f32, f32),
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            azimuth: 0.4,
            elevation: 0.3,
            distance: 3.0,
            target: Vec3::ZERO,
            dragging: false,
            last_mouse: (0.0, 0.0),
        }
    }
}

impl OrbitCamera {
    /// Compute the eye position from spherical coordinates.
    pub fn eye_position(&self) -> Vec3 {
        let cos_elev = self.elevation.cos();
        let x = self.distance * cos_elev * self.azimuth.sin();
        let y = self.distance * self.elevation.sin();
        let z = self.distance * cos_elev * self.azimuth.cos();
        self.target + Vec3::new(x, y, z)
    }

    /// View matrix (right-handed, looking at target).
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye_position(), self.target, Vec3::Y)
    }

    /// Perspective projection matrix.
    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0)
    }

    pub fn on_mouse_down(&mut self, x: f32, y: f32) {
        self.dragging = true;
        self.last_mouse = (x, y);
    }

    pub fn on_mouse_up(&mut self) {
        self.dragging = false;
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        if !self.dragging {
            return;
        }
        let dx = x - self.last_mouse.0;
        let dy = y - self.last_mouse.1;
        self.last_mouse = (x, y);

        self.azimuth += dx * 0.01;
        self.elevation += dy * 0.01;

        // Clamp elevation to avoid gimbal lock
        let limit = std::f32::consts::FRAC_PI_2 - 0.01;
        self.elevation = self.elevation.clamp(-limit, limit);
    }

    pub fn on_wheel(&mut self, delta: f32) {
        self.distance *= 1.0 + delta * 0.001;
        self.distance = self.distance.clamp(0.5, 50.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_eye_position() {
        let cam = OrbitCamera::default();
        let eye = cam.eye_position();
        // Should be offset from origin
        assert!(eye.length() > 0.0);
        assert!((eye.length() - cam.distance).abs() < 0.01);
    }

    #[test]
    fn test_elevation_clamp() {
        let mut cam = OrbitCamera::default();
        cam.dragging = true;
        cam.last_mouse = (0.0, 0.0);
        // Large vertical drag
        cam.on_mouse_move(0.0, 10000.0);
        let limit = std::f32::consts::FRAC_PI_2 - 0.01;
        assert!(cam.elevation <= limit);
        assert!(cam.elevation >= -limit);
    }

    #[test]
    fn test_zoom() {
        let mut cam = OrbitCamera::default();
        let original = cam.distance;
        cam.on_wheel(100.0);
        assert!(cam.distance > original);
        cam.on_wheel(-200.0);
        assert!(cam.distance < original);
    }
}
