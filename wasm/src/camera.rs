use crate::math::Vec3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn center(&self) -> Vec3 {
        self.min.add(self.max).mul(0.5)
    }

    pub fn radius(&self) -> f32 {
        // bounding sphere radius from AABB
        self.max.sub(self.min).length() * 0.5
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Camera {
    /// Orbit target (what we rotate around).
    pub target: Vec3,
    /// Distance from target.
    pub distance: f32,
    /// Yaw (around +Y), radians.
    pub yaw: f32,
    /// Pitch (around +X in camera-local), radians.
    pub pitch: f32,
    /// Vertical field-of-view (radians).
    pub fovy: f32,
    /// Near/far clip.
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            target: Vec3::new(0.0, 0.0, 0.0),
            distance: 2.0,
            yaw: 0.0,
            pitch: 0.0,
            fovy: 45_f32.to_radians(),
            znear: 0.01,
            zfar: 1000.0,
        }
    }

    pub fn eye(&self) -> Vec3 {
        // Orbit around target using yaw/pitch.
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();

        // Forward direction from target to eye (spherical coordinates).
        let dir = Vec3::new(cp * sy, sp, cp * cy);
        self.target.add(dir.mul(self.distance))
    }

    pub fn view_up(&self) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-1.54, 1.54); // ~ +/- 88.2 deg
    }

    pub fn zoom(&mut self, factor: f32) {
        // factor > 1 zooms out, < 1 zooms in
        self.distance = (self.distance * factor).clamp(0.05, 1.0e6);
    }

    pub fn pan(&mut self, right: f32, up: f32) {
        // Pan in view plane: move target by camera right/up vectors.
        let eye = self.eye();
        let forward = self.target.sub(eye).normalize();
        let world_up = self.view_up();
        let cam_right = forward.cross(world_up).normalize();
        let cam_up = cam_right.cross(forward).normalize();

        self.target = self.target.add(cam_right.mul(right)).add(cam_up.mul(up));
    }

    pub fn fit_to_bounds(&mut self, bounds: Bounds, aspect: f32) {
        self.target = bounds.center();
        let r = bounds.radius().max(1e-4);

        // Distance so that bounding sphere fits vertically; adjust for aspect.
        let tan_half_fovy = (self.fovy * 0.5).tan();
        let mut dist = r / tan_half_fovy;

        // If viewport is portrait/narrow, horizontal FOV is smaller -> need more distance.
        let tan_half_fovx = tan_half_fovy * aspect;
        if tan_half_fovx > 0.0 {
            let dist_x = r / tan_half_fovx;
            dist = dist.max(dist_x);
        }

        // Add a little padding.
        self.distance = dist * 1.15;
        self.znear = (self.distance - r * 2.5).max(0.001);
        self.zfar = (self.distance + r * 2.5).max(self.znear + 1.0);
    }
}

