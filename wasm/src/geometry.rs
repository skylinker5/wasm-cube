use crate::camera::Bounds;
use crate::math::Vec3;

/// Simple mesh: positions (x,y,z) and optional triangle indices.
#[derive(Debug, Clone)]
pub(crate) struct Mesh {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u16>,
    pub bounds: Bounds,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Primitive {
    Triangle,
    Cube,
    Cylinder,
    Sphere,
    Torus,
}

impl Primitive {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "triangle" => Some(Self::Triangle),
            "cube" => Some(Self::Cube),
            "cylinder" => Some(Self::Cylinder),
            "sphere" => Some(Self::Sphere),
            "torus" => Some(Self::Torus),
            _ => None,
        }
    }
}

pub(crate) fn make_primitive(p: Primitive) -> Mesh {
    match p {
        Primitive::Triangle => triangle(),
        Primitive::Cube => cube(),
        Primitive::Cylinder => cylinder(0.5, 1.0, 32),
        Primitive::Sphere => sphere(0.5, 32, 16),
        Primitive::Torus => torus(0.6, 0.2, 32, 16),
    }
}

pub(crate) fn triangle() -> Mesh {
    let positions = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    mesh_from_positions_indices(positions, vec![])
}

pub(crate) fn cube() -> Mesh {
    // Unit cube centered at origin, size 1.0.
    let p = [
        (-0.5f32, -0.5f32, -0.5f32), // 0
        (0.5f32, -0.5f32, -0.5f32),  // 1
        (0.5f32, 0.5f32, -0.5f32),   // 2
        (-0.5f32, 0.5f32, -0.5f32),  // 3
        (-0.5f32, -0.5f32, 0.5f32),  // 4
        (0.5f32, -0.5f32, 0.5f32),   // 5
        (0.5f32, 0.5f32, 0.5f32),    // 6
        (-0.5f32, 0.5f32, 0.5f32),   // 7
    ];
    let mut positions = Vec::with_capacity(8 * 3);
    for (x, y, z) in p {
        positions.extend_from_slice(&[x, y, z]);
    }

    // 12 triangles (two per face), CCW winding.
    let indices: Vec<u16> = vec![
        // back (-z)
        0, 1, 2, 0, 2, 3, //
        // front (+z)
        4, 6, 5, 4, 7, 6, //
        // left (-x)
        0, 3, 7, 0, 7, 4, //
        // right (+x)
        1, 5, 6, 1, 6, 2, //
        // bottom (-y)
        0, 4, 5, 0, 5, 1, //
        // top (+y)
        3, 2, 6, 3, 6, 7, //
    ];

    mesh_from_positions_indices(positions, indices)
}

pub(crate) fn cylinder(radius: f32, height: f32, segments: u32) -> Mesh {
    let segments = segments.max(3) as usize;
    let half_h = height * 0.5;

    // Layout:
    // - ring vertices: 2 per segment (bottom/top)
    // - cap centers: 2 vertices
    let mut positions = Vec::with_capacity((segments * 2 + 2) * 3);

    for i in 0..segments {
        let t = (i as f32) * std::f32::consts::TAU / (segments as f32);
        let (s, c) = t.sin_cos();
        let x = c * radius;
        let z = s * radius;
        // bottom
        positions.extend_from_slice(&[x, -half_h, z]);
        // top
        positions.extend_from_slice(&[x, half_h, z]);
    }

    let bottom_center_idx = (segments * 2) as u16;
    positions.extend_from_slice(&[0.0, -half_h, 0.0]);
    let top_center_idx = (segments * 2 + 1) as u16;
    positions.extend_from_slice(&[0.0, half_h, 0.0]);

    let mut indices: Vec<u16> = Vec::new();

    // sides (two triangles per quad)
    for i in 0..segments {
        let j = (i + 1) % segments;
        let b0 = (i * 2) as u16;
        let t0 = (i * 2 + 1) as u16;
        let b1 = (j * 2) as u16;
        let t1 = (j * 2 + 1) as u16;

        indices.extend_from_slice(&[b0, b1, t1, b0, t1, t0]);
    }

    // bottom cap (fan) - winding so outside faces outwards
    for i in 0..segments {
        let j = (i + 1) % segments;
        let b0 = (i * 2) as u16;
        let b1 = (j * 2) as u16;
        indices.extend_from_slice(&[bottom_center_idx, b1, b0]);
    }

    // top cap (fan)
    for i in 0..segments {
        let j = (i + 1) % segments;
        let t0 = (i * 2 + 1) as u16;
        let t1 = (j * 2 + 1) as u16;
        indices.extend_from_slice(&[top_center_idx, t0, t1]);
    }

    mesh_from_positions_indices(positions, indices)
}

pub(crate) fn sphere(radius: f32, segments_u: u32, segments_v: u32) -> Mesh {
    // longitude (u): 0..2pi, latitude (v): 0..pi
    let u = segments_u.max(3) as usize;
    let v = segments_v.max(2) as usize;

    let mut positions: Vec<f32> = Vec::with_capacity((u + 1) * (v + 1) * 3);
    for iy in 0..=v {
        let fy = iy as f32 / (v as f32);
        let theta = fy * std::f32::consts::PI; // 0..pi
        let (st, ct) = theta.sin_cos();
        for ix in 0..=u {
            let fx = ix as f32 / (u as f32);
            let phi = fx * std::f32::consts::TAU; // 0..2pi
            let (sp, cp) = phi.sin_cos();

            let x = cp * st * radius;
            let y = ct * radius;
            let z = sp * st * radius;
            positions.extend_from_slice(&[x, y, z]);
        }
    }

    let stride = (u + 1) as u16;
    let mut indices: Vec<u16> = Vec::with_capacity(u * v * 6);
    for iy in 0..v {
        for ix in 0..u {
            let a = (iy as u16) * stride + (ix as u16);
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            // two triangles per quad
            indices.extend_from_slice(&[a, c, d, a, d, b]);
        }
    }

    mesh_from_positions_indices(positions, indices)
}

pub(crate) fn torus(major_radius: f32, minor_radius: f32, segments_u: u32, segments_v: u32) -> Mesh {
    // u: around the hole, v: around the tube
    let u = segments_u.max(3) as usize;
    let v = segments_v.max(3) as usize;

    let mut positions: Vec<f32> = Vec::with_capacity((u + 1) * (v + 1) * 3);
    for iu in 0..=u {
        let fu = iu as f32 / (u as f32);
        let theta = fu * std::f32::consts::TAU;
        let (st, ct) = theta.sin_cos();

        for iv in 0..=v {
            let fv = iv as f32 / (v as f32);
            let phi = fv * std::f32::consts::TAU;
            let (sp, cp) = phi.sin_cos();

            let r = major_radius + minor_radius * cp;
            let x = ct * r;
            let y = minor_radius * sp;
            let z = st * r;
            positions.extend_from_slice(&[x, y, z]);
        }
    }

    let stride = (v + 1) as u16;
    let mut indices: Vec<u16> = Vec::with_capacity(u * v * 6);
    for iu in 0..u {
        for iv in 0..v {
            let a = (iu as u16) * stride + (iv as u16);
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, d, a, d, b]);
        }
    }

    mesh_from_positions_indices(positions, indices)
}

fn mesh_from_positions_indices(positions: Vec<f32>, indices: Vec<u16>) -> Mesh {
    let bounds = compute_bounds(&positions);
    let normals = compute_normals(&positions, &indices);
    Mesh {
        positions,
        normals,
        indices,
        bounds,
    }
}

fn compute_bounds(positions: &[f32]) -> Bounds {
    if positions.len() < 3 {
        return Bounds::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    }
    let mut min = Vec3::new(positions[0], positions[1], positions[2]);
    let mut max = Vec3::new(positions[0], positions[1], positions[2]);
    for v in positions.chunks_exact(3) {
        let x = v[0];
        let y = v[1];
        let z = v[2];
        min.x = min.x.min(x);
        min.y = min.y.min(y);
        min.z = min.z.min(z);
        max.x = max.x.max(x);
        max.y = max.y.max(y);
        max.z = max.z.max(z);
    }
    Bounds::new(min, max)
}

/// Compute per-vertex normals by averaging adjacent triangle normals.
/// Handles both indexed and non-indexed geometry.
fn compute_normals(positions: &[f32], indices: &[u16]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len()];

    if indices.is_empty() {
        // Non-indexed: assume triangles laid out sequentially.
        for (tri_idx, tri) in positions.chunks_exact(9).enumerate() {
            let a = Vec3::new(tri[0], tri[1], tri[2]);
            let b = Vec3::new(tri[3], tri[4], tri[5]);
            let c = Vec3::new(tri[6], tri[7], tri[8]);
            let n = b.sub(a).cross(c.sub(a)).normalize();
            let base = tri_idx * 9;
            for v in 0..3 {
                let dst = base + v * 3;
                normals[dst] += n.x;
                normals[dst + 1] += n.y;
                normals[dst + 2] += n.z;
            }
        }
    } else {
        // Indexed: accumulate face normals for each referenced vertex.
        for idx in indices.chunks_exact(3) {
            let ia = idx[0] as usize * 3;
            let ib = idx[1] as usize * 3;
            let ic = idx[2] as usize * 3;
            if ic + 2 >= positions.len() {
                continue;
            }
            let a = Vec3::new(positions[ia], positions[ia + 1], positions[ia + 2]);
            let b = Vec3::new(positions[ib], positions[ib + 1], positions[ib + 2]);
            let c = Vec3::new(positions[ic], positions[ic + 1], positions[ic + 2]);
            let n = b.sub(a).cross(c.sub(a)).normalize();
            for &i in &[ia, ib, ic] {
                normals[i] += n.x;
                normals[i + 1] += n.y;
                normals[i + 2] += n.z;
            }
        }
    }

    // Normalize the accumulated normals; default to +Z when degenerate.
    for n in normals.chunks_exact_mut(3) {
        let v = Vec3::new(n[0], n[1], n[2]).normalize();
        if v.length() <= 1e-8 {
            n[0] = 0.0;
            n[1] = 0.0;
            n[2] = 1.0;
        } else {
            n[0] = v.x;
            n[1] = v.y;
            n[2] = v.z;
        }
    }

    normals
}

