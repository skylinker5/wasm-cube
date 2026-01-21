#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len <= 1e-8 {
            self
        } else {
            self.mul(1.0 / len)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Mat4 {
    /// Column-major 4x4 matrix (WebGL expects column-major).
    pub m: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            m: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }

    pub fn mul(self, b: Mat4) -> Mat4 {
        // Column-major multiplication: out = self * b
        let a = self.m;
        let b = b.m;
        let mut out = [0.0f32; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = a[0 * 4 + row] * b[col * 4 + 0]
                    + a[1 * 4 + row] * b[col * 4 + 1]
                    + a[2 * 4 + row] * b[col * 4 + 2]
                    + a[3 * 4 + row] * b[col * 4 + 3];
            }
        }
        Mat4 { m: out }
    }

    pub fn translation(v: Vec3) -> Mat4 {
        let mut m = Mat4::identity().m;
        m[12] = v.x;
        m[13] = v.y;
        m[14] = v.z;
        Mat4 { m }
    }

    pub fn perspective(fovy_radians: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
        let f = 1.0 / (0.5 * fovy_radians).tan();
        let nf = 1.0 / (znear - zfar);
        Mat4 {
            m: [
                f / aspect,
                0.0,
                0.0,
                0.0,
                0.0,
                f,
                0.0,
                0.0,
                0.0,
                0.0,
                (zfar + znear) * nf,
                -1.0,
                0.0,
                0.0,
                (2.0 * zfar * znear) * nf,
                0.0,
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        // Column-major
        Mat4 {
            m: [
                s.x,
                u.x,
                -f.x,
                0.0,
                s.y,
                u.y,
                -f.y,
                0.0,
                s.z,
                u.z,
                -f.z,
                0.0,
                -s.dot(eye),
                -u.dot(eye),
                f.dot(eye),
                1.0,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    fn assert_vec3_approx(a: Vec3, b: Vec3, eps: f32) {
        assert!(
            approx_eq(a.x, b.x, eps) && approx_eq(a.y, b.y, eps) && approx_eq(a.z, b.z, eps),
            "Vec3 mismatch: left={a:?} right={b:?} eps={eps}"
        );
    }

    #[test]
    fn vec3_dot_cross_sanity() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);

        assert!(approx_eq(x.dot(y), 0.0, 1e-6));
        assert_vec3_approx(x.cross(y), Vec3::new(0.0, 0.0, 1.0), 1e-6);
        assert_vec3_approx(y.cross(x), Vec3::new(0.0, 0.0, -1.0), 1e-6);
    }

    #[test]
    fn vec3_normalize_handles_zero_and_unit() {
        let z = Vec3::new(0.0, 0.0, 0.0).normalize();
        assert_vec3_approx(z, Vec3::new(0.0, 0.0, 0.0), 0.0);

        let u = Vec3::new(0.0, 3.0, 4.0).normalize();
        assert!(approx_eq(u.length(), 1.0, 1e-6));
    }

    #[test]
    fn mat4_identity_mul_is_noop() {
        let i = Mat4::identity();
        let t = Mat4::translation(Vec3::new(3.0, -2.0, 5.0));

        assert_eq!(i.mul(t).m, t.m);
        assert_eq!(t.mul(i).m, t.m);
    }

    #[test]
    fn mat4_translation_is_column_major() {
        let t = Mat4::translation(Vec3::new(7.0, 8.0, 9.0));
        // Column-major translation lives at indices 12,13,14
        assert!(approx_eq(t.m[12], 7.0, 0.0));
        assert!(approx_eq(t.m[13], 8.0, 0.0));
        assert!(approx_eq(t.m[14], 9.0, 0.0));
        assert!(approx_eq(t.m[15], 1.0, 0.0));
    }

    #[test]
    fn mat4_look_at_builds_orthonormal_basis() {
        let eye = Vec3::new(0.0, 0.0, 5.0);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        let m = Mat4::look_at(eye, target, up).m;
        // Extract basis vectors from column-major matrix:
        // s = first column, u = second, -f = third.
        let s = Vec3::new(m[0], m[4], m[8]);
        let u = Vec3::new(m[1], m[5], m[9]);
        let neg_f = Vec3::new(m[2], m[6], m[10]);

        assert!(approx_eq(s.length(), 1.0, 1e-5));
        assert!(approx_eq(u.length(), 1.0, 1e-5));
        assert!(approx_eq(neg_f.length(), 1.0, 1e-5));

        assert!(approx_eq(s.dot(u), 0.0, 1e-5));
        assert!(approx_eq(s.dot(neg_f), 0.0, 1e-5));
        assert!(approx_eq(u.dot(neg_f), 0.0, 1e-5));
    }
}

