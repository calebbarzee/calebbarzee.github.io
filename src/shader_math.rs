/// Pure Rust ports of GLSL shader math functions for testing.
/// These must match the behavior of the corresponding GLSL code.

/// Fullscreen triangle vertex position from gl_VertexID.
/// Matches scene.vert / fullscreen_triangle.vert logic.
pub fn fullscreen_triangle_position(vertex_id: u32) -> (f32, f32) {
    let x = ((vertex_id & 1) << 2) as f32 - 1.0;
    let y = ((vertex_id & 2) << 1) as f32 - 1.0;
    (x, y)
}

/// CRT barrel distortion. Matches ascii.frag `crtCurve()`.
pub fn crt_curve(uv: (f32, f32)) -> (f32, f32) {
    let mut x = uv.0 * 2.0 - 1.0;
    let mut y = uv.1 * 2.0 - 1.0;
    let offset_x = (y.abs()) / 5.0;
    let offset_y = (x.abs()) / 4.0;
    x = x + x * offset_x * offset_x;
    y = y + y * offset_y * offset_y;
    x = x * 0.5 + 0.5;
    y = y * 0.5 + 0.5;
    (x, y)
}

/// BT.601 luminance calculation. Matches `dot(color, vec3(0.299, 0.587, 0.114))`.
pub fn luminance(r: f32, g: f32, b: f32) -> f32 {
    r * 0.299 + g * 0.587 + b * 0.114
}

/// GLSL fract() equivalent: always returns [0, 1), unlike Rust's f32::fract()
/// which can return negative values.
fn glsl_fract(x: f32) -> f32 {
    x - x.floor()
}

/// Hash function matching scene.frag `hash()`.
pub fn hash(p: (f32, f32)) -> f32 {
    let dot = p.0 * 127.1 + p.1 * 311.7;
    glsl_fract(dot.sin() * 43758.5453)
}

/// Value noise matching scene.frag `noise()`.
pub fn noise(p: (f32, f32)) -> f32 {
    let ix = p.0.floor();
    let iy = p.1.floor();
    let fx = p.0 - ix;
    let fy = p.1 - iy;
    // Hermite smoothing
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);

    let a = hash((ix, iy));
    let b = hash((ix + 1.0, iy));
    let c = hash((ix, iy + 1.0));
    let d = hash((ix + 1.0, iy + 1.0));

    let ab = a + (b - a) * sx;
    let cd = c + (d - c) * sx;
    ab + (cd - ab) * sy
}

/// FBM (fractal Brownian motion) matching scene.frag `fbm()`.
pub fn fbm(p: (f32, f32)) -> f32 {
    let mut v = 0.0_f32;
    let mut a = 0.5_f32;
    let mut px = p.0;
    let mut py = p.1;
    // Rotation matrix: mat2(0.8, 0.6, -0.6, 0.8)
    for _ in 0..5 {
        v += a * noise((px, py));
        let new_px = 0.8 * px + 0.6 * py;
        let new_py = -0.6 * px + 0.8 * py;
        px = new_px * 2.0;
        py = new_py * 2.0;
        a *= 0.5;
    }
    v
}

/// 5x5 bit-pattern encoded ASCII characters.
/// Each i32 encodes 25 bits: bit i = pixel at (i%5, i/5).
pub const CHARS: [i32; 10] = [
    0,         // ' ' (space)
    4329604,   // '.'
    14749384,  // ':'
    4539716,   // '-'
    11512810,  // '+'
    32641156,  // '='
    15252014,  // '*'
    11983725,  // '#'
    33061407,  // '@'
    33554431,  // block (full)
];

/// Get character pixel at (pos_x, pos_y) for character index.
/// Matches ascii.frag `getChar()`.
pub fn get_char(idx: usize, pos_x: f32, pos_y: f32) -> f32 {
    let x = ((pos_x * 5.0) as i32).clamp(0, 4);
    let y = ((pos_y * 5.0) as i32).clamp(0, 4);
    let bit = y * 5 + x;
    let pattern = CHARS[idx];
    ((pattern >> bit) & 1) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Fullscreen triangle tests ──────────────────────────

    #[test]
    fn fullscreen_triangle_vertex_0() {
        let (x, y) = fullscreen_triangle_position(0);
        assert_eq!((x, y), (-1.0, -1.0));
    }

    #[test]
    fn fullscreen_triangle_vertex_1() {
        let (x, y) = fullscreen_triangle_position(1);
        assert_eq!((x, y), (3.0, -1.0));
    }

    #[test]
    fn fullscreen_triangle_vertex_2() {
        let (x, y) = fullscreen_triangle_position(2);
        assert_eq!((x, y), (-1.0, 3.0));
    }

    // ── CRT barrel distortion tests ────────────────────────

    #[test]
    fn crt_center_maps_to_center() {
        let (x, y) = crt_curve((0.5, 0.5));
        assert!((x - 0.5).abs() < 0.001, "center x: {}", x);
        assert!((y - 0.5).abs() < 0.001, "center y: {}", y);
    }

    #[test]
    fn crt_symmetric_horizontal() {
        let left = crt_curve((0.25, 0.5));
        let right = crt_curve((0.75, 0.5));
        // x positions should be symmetric around 0.5
        assert!(
            ((left.0 + right.0) / 2.0 - 0.5).abs() < 0.001,
            "left={:?}, right={:?}",
            left,
            right
        );
        // y positions should be equal
        assert!(
            (left.1 - right.1).abs() < 0.001,
            "left.y={}, right.y={}",
            left.1,
            right.1
        );
    }

    #[test]
    fn crt_corners_displaced_outward() {
        // A corner like (0.1, 0.1) should be pushed further from center
        let (cx, cy) = crt_curve((0.1, 0.1));
        // In CRT distortion, corners get pushed outward (farther from 0.5, 0.5)
        // The distorted point should be farther from center than the original
        let orig_dist = ((0.1 - 0.5_f32).powi(2) + (0.1 - 0.5_f32).powi(2)).sqrt();
        let crt_dist = ((cx - 0.5).powi(2) + (cy - 0.5).powi(2)).sqrt();
        assert!(
            crt_dist > orig_dist,
            "CRT should push corners outward: orig_dist={}, crt_dist={}",
            orig_dist,
            crt_dist
        );
    }

    // ── Luminance tests ────────────────────────────────────

    #[test]
    fn luminance_white() {
        let l = luminance(1.0, 1.0, 1.0);
        assert!((l - 1.0).abs() < 0.001, "white luminance: {}", l);
    }

    #[test]
    fn luminance_black() {
        let l = luminance(0.0, 0.0, 0.0);
        assert!((l - 0.0).abs() < 0.001, "black luminance: {}", l);
    }

    #[test]
    fn luminance_pure_green() {
        let l = luminance(0.0, 1.0, 0.0);
        assert!((l - 0.587).abs() < 0.001, "green luminance: {}", l);
    }

    #[test]
    fn luminance_pure_red() {
        let l = luminance(1.0, 0.0, 0.0);
        assert!((l - 0.299).abs() < 0.001, "red luminance: {}", l);
    }

    #[test]
    fn luminance_pure_blue() {
        let l = luminance(0.0, 0.0, 1.0);
        assert!((l - 0.114).abs() < 0.001, "blue luminance: {}", l);
    }

    // ── ASCII character bit pattern tests ──────────────────

    #[test]
    fn space_char_all_zero() {
        // Space (index 0) should be all zeros
        for y in 0..5 {
            for x in 0..5 {
                let px = (x as f32 + 0.5) / 5.0;
                let py = (y as f32 + 0.5) / 5.0;
                assert_eq!(
                    get_char(0, px, py),
                    0.0,
                    "space should be blank at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn full_block_all_one() {
        // Full block (index 9) should be all ones
        for y in 0..5 {
            for x in 0..5 {
                let px = (x as f32 + 0.5) / 5.0;
                let py = (y as f32 + 0.5) / 5.0;
                assert_eq!(
                    get_char(9, px, py),
                    1.0,
                    "full block should be filled at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn full_block_pattern_value() {
        // 33554431 = 2^25 - 1 = all 25 bits set
        assert_eq!(CHARS[9], (1 << 25) - 1);
    }

    #[test]
    fn dot_char_has_some_pixels() {
        // The dot character (index 1) should have some pixels on and some off
        let mut on_count = 0;
        let mut off_count = 0;
        for y in 0..5 {
            for x in 0..5 {
                let px = (x as f32 + 0.5) / 5.0;
                let py = (y as f32 + 0.5) / 5.0;
                if get_char(1, px, py) > 0.5 {
                    on_count += 1;
                } else {
                    off_count += 1;
                }
            }
        }
        assert!(on_count > 0, "dot should have some pixels on");
        assert!(off_count > 0, "dot should have some pixels off");
    }

    // ── Noise/FBM property tests ───────────────────────────

    #[test]
    fn hash_output_in_range() {
        // Hash should produce values that, when fract'd, are in [0, 1)
        for i in 0..100 {
            for j in 0..100 {
                let h = hash((i as f32 * 0.1, j as f32 * 0.1));
                assert!(
                    h >= 0.0 && h < 1.0,
                    "hash out of range at ({}, {}): {}",
                    i,
                    j,
                    h
                );
            }
        }
    }

    #[test]
    fn noise_output_bounded() {
        // Noise is a bilinear interpolation of hash values in [0,1),
        // so it should also be in [0, 1)
        for i in 0..50 {
            for j in 0..50 {
                let n = noise((i as f32 * 0.3, j as f32 * 0.3));
                assert!(
                    n >= 0.0 && n <= 1.0,
                    "noise out of [0,1] at ({}, {}): {}",
                    i,
                    j,
                    n
                );
            }
        }
    }

    #[test]
    fn noise_is_continuous() {
        // Nearby inputs should produce nearby outputs
        let base = noise((1.0, 1.0));
        let nearby = noise((1.001, 1.001));
        let diff = (base - nearby).abs();
        assert!(
            diff < 0.1,
            "noise should be continuous: base={}, nearby={}, diff={}",
            base,
            nearby,
            diff
        );
    }

    #[test]
    fn fbm_output_bounded() {
        // FBM sums diminishing amplitudes of noise in [0,1]:
        // 0.5 + 0.25 + 0.125 + 0.0625 + 0.03125 = 0.96875
        for i in 0..20 {
            for j in 0..20 {
                let f = fbm((i as f32 * 0.5, j as f32 * 0.5));
                assert!(
                    f >= 0.0 && f <= 1.0,
                    "fbm out of range at ({}, {}): {}",
                    i,
                    j,
                    f
                );
            }
        }
    }

    // ── glam integration sanity tests ──────────────────────

    #[test]
    fn glam_perspective_basic() {
        use glam::Mat4;
        let proj = Mat4::perspective_rh_gl(
            std::f32::consts::FRAC_PI_4, // 45 degree fov
            1.0,                          // aspect ratio
            0.1,                          // near
            100.0,                        // far
        );
        // A point at the near plane center should map to z = -1 in clip space
        let near_point = proj * glam::Vec4::new(0.0, 0.0, -0.1, 1.0);
        let ndc_z = near_point.z / near_point.w;
        assert!(
            (ndc_z - (-1.0)).abs() < 0.01,
            "near plane should map to NDC z=-1, got {}",
            ndc_z
        );
    }

    #[test]
    fn glam_look_at_eye_at_origin() {
        use glam::{Mat4, Vec3};
        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 5.0), // eye
            Vec3::ZERO,                // target
            Vec3::Y,                   // up
        );
        // The eye position transformed by the view matrix should be at origin
        let eye_in_view = view * glam::Vec4::new(0.0, 0.0, 5.0, 1.0);
        assert!(
            eye_in_view.x.abs() < 0.001
                && eye_in_view.y.abs() < 0.001
                && eye_in_view.z.abs() < 0.001,
            "eye should be at view-space origin, got {:?}",
            eye_in_view
        );
    }

    #[test]
    fn glam_look_at_target_on_negative_z() {
        use glam::{Mat4, Vec3};
        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::Y,
        );
        // Target (origin) should be at negative z in view space
        let target_in_view = view * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!(
            target_in_view.z < 0.0,
            "target should be at negative z in view space, got {}",
            target_in_view.z
        );
    }
}
