/// STL file parser supporting both binary and ASCII formats.

#[derive(Debug, Clone)]
pub struct StlMesh {
    /// Flat array of vertex positions: [x0,y0,z0, x1,y1,z1, ...]
    pub positions: Vec<f32>,
    /// Flat array of per-vertex normals: [nx0,ny0,nz0, ...]
    pub normals: Vec<f32>,
    /// Bounding box min corner
    pub min: [f32; 3],
    /// Bounding box max corner
    pub max: [f32; 3],
}

impl StlMesh {
    /// Center of the bounding box.
    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Maximum extent (largest side of the bounding box).
    pub fn extent(&self) -> f32 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        dx.max(dy).max(dz)
    }

    /// Number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.positions.len() / 9
    }
}

/// Parse an STL file from raw bytes (auto-detects binary vs ASCII).
pub fn parse_stl(data: &[u8]) -> Result<StlMesh, String> {
    if is_ascii_stl(data) {
        parse_ascii_stl(data)
    } else {
        parse_binary_stl(data)
    }
}

fn is_ascii_stl(data: &[u8]) -> bool {
    if data.len() < 6 {
        return false;
    }
    let header = std::str::from_utf8(&data[..6.min(data.len())]).unwrap_or("");
    header.starts_with("solid ")
        && data.windows(5).any(|w| w == b"facet")
}

/// Parse a binary STL file.
/// Format: 80-byte header, 4-byte u32 triangle count, 50 bytes per triangle.
fn parse_binary_stl(data: &[u8]) -> Result<StlMesh, String> {
    if data.len() < 84 {
        return Err("Binary STL too short for header".into());
    }

    let tri_count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;
    let expected_size = 84 + tri_count * 50;
    if data.len() < expected_size {
        return Err(format!(
            "Binary STL truncated: expected {} bytes, got {}",
            expected_size,
            data.len()
        ));
    }

    let mut positions = Vec::with_capacity(tri_count * 9);
    let mut normals = Vec::with_capacity(tri_count * 9);
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];

    for i in 0..tri_count {
        let offset = 84 + i * 50;
        let nx = read_f32_le(data, offset);
        let ny = read_f32_le(data, offset + 4);
        let nz = read_f32_le(data, offset + 8);

        for v in 0..3 {
            let vo = offset + 12 + v * 12;
            let x = read_f32_le(data, vo);
            let y = read_f32_le(data, vo + 4);
            let z = read_f32_le(data, vo + 8);

            positions.push(x);
            positions.push(y);
            positions.push(z);

            normals.push(nx);
            normals.push(ny);
            normals.push(nz);

            update_bounds(&mut min, &mut max, x, y, z);
        }
    }

    Ok(StlMesh {
        positions,
        normals,
        min,
        max,
    })
}

/// Parse an ASCII STL file.
fn parse_ascii_stl(data: &[u8]) -> Result<StlMesh, String> {
    let text = std::str::from_utf8(data).map_err(|e| format!("Invalid UTF-8: {}", e))?;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    let mut current_normal = [0.0f32; 3];

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("facet normal ") {
            let parts: Vec<f32> = rest
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() == 3 {
                current_normal = [parts[0], parts[1], parts[2]];
            }
        } else if let Some(rest) = trimmed.strip_prefix("vertex ") {
            let parts: Vec<f32> = rest
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() == 3 {
                let (x, y, z) = (parts[0], parts[1], parts[2]);
                positions.push(x);
                positions.push(y);
                positions.push(z);
                normals.push(current_normal[0]);
                normals.push(current_normal[1]);
                normals.push(current_normal[2]);
                update_bounds(&mut min, &mut max, x, y, z);
            }
        }
    }

    if positions.is_empty() {
        return Err("No vertices found in ASCII STL".into());
    }

    Ok(StlMesh {
        positions,
        normals,
        min,
        max,
    })
}

fn read_f32_le(data: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn update_bounds(min: &mut [f32; 3], max: &mut [f32; 3], x: f32, y: f32, z: f32) {
    min[0] = min[0].min(x);
    min[1] = min[1].min(y);
    min[2] = min[2].min(z);
    max[0] = max[0].max(x);
    max[1] = max[1].max(y);
    max[2] = max[2].max(z);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_binary_stl(triangles: &[([f32; 3], [[f32; 3]; 3])]) -> Vec<u8> {
        let mut data = vec![0u8; 80]; // header
        let count = triangles.len() as u32;
        data.extend_from_slice(&count.to_le_bytes());

        for (normal, verts) in triangles {
            for val in normal {
                data.extend_from_slice(&val.to_le_bytes());
            }
            for vert in verts {
                for val in vert {
                    data.extend_from_slice(&val.to_le_bytes());
                }
            }
            data.extend_from_slice(&[0u8; 2]); // attribute byte count
        }
        data
    }

    #[test]
    fn test_parse_binary_single_triangle() {
        let data = make_binary_stl(&[(
            [0.0, 0.0, 1.0],
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        )]);

        let mesh = parse_binary_stl(&data).unwrap();
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.positions.len(), 9);
        assert_eq!(mesh.normals.len(), 9);
        // All 3 vertices share the face normal
        assert_eq!(mesh.normals[0..3], [0.0, 0.0, 1.0]);
        assert_eq!(mesh.normals[3..6], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_parse_ascii_stl() {
        let ascii = b"solid test
  facet normal 0 0 1
    outer loop
      vertex 0 0 0
      vertex 1 0 0
      vertex 0 1 0
    endloop
  endfacet
endsolid test";

        let mesh = parse_ascii_stl(ascii).unwrap();
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.positions, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_auto_detect_binary() {
        let data = make_binary_stl(&[(
            [0.0, 1.0, 0.0],
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 1.0, 0.0]],
        )]);
        let mesh = parse_stl(&data).unwrap();
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_auto_detect_ascii() {
        let ascii = b"solid cube
  facet normal 0 0 1
    outer loop
      vertex 0 0 0
      vertex 1 0 0
      vertex 0 1 0
    endloop
  endfacet
endsolid cube";
        let mesh = parse_stl(ascii).unwrap();
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_bounding_box() {
        let data = make_binary_stl(&[(
            [0.0, 0.0, 1.0],
            [[-1.0, -2.0, -3.0], [4.0, 5.0, 6.0], [0.0, 0.0, 0.0]],
        )]);
        let mesh = parse_stl(&data).unwrap();
        assert_eq!(mesh.min, [-1.0, -2.0, -3.0]);
        assert_eq!(mesh.max, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_center_and_extent() {
        let data = make_binary_stl(&[(
            [0.0, 0.0, 1.0],
            [[0.0, 0.0, 0.0], [10.0, 4.0, 6.0], [5.0, 2.0, 3.0]],
        )]);
        let mesh = parse_stl(&data).unwrap();
        assert_eq!(mesh.center(), [5.0, 2.0, 3.0]);
        assert_eq!(mesh.extent(), 10.0);
    }

    #[test]
    fn test_binary_too_short() {
        let data = vec![0u8; 50];
        assert!(parse_binary_stl(&data).is_err());
    }

    #[test]
    fn test_ascii_no_vertices() {
        let ascii = b"solid empty\nendsolid empty";
        assert!(parse_ascii_stl(ascii).is_err());
    }
}
