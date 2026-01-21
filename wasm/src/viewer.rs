use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::camera::{Bounds, Camera};
use crate::geometry::{make_primitive, Primitive};
use crate::math::{Mat4, Vec3};
use crate::renderer::Renderer;

#[wasm_bindgen]
pub struct Viewer {
    renderer: Renderer,
    camera: Camera,
    width: i32,
    height: i32,
    bounds: Bounds,
}

#[wasm_bindgen]
impl Viewer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Viewer, JsValue> {
        let gl = crate::get_webgl_context(&canvas)?;
        let width = canvas.width() as i32;
        let height = canvas.height() as i32;

        let mut renderer = Renderer::new(gl)?;
        let camera = Camera::new();

        let mesh = make_primitive(Primitive::Triangle);
        renderer.set_mesh(&mesh);
        let bounds = mesh.bounds;

        let mut viewer = Viewer {
            renderer,
            camera,
            width,
            height,
            bounds,
        };
        viewer.fit_to_view();
        viewer.draw();
        Ok(viewer)
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    pub fn set_bounds(&mut self, min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) {
        self.bounds = Bounds::new(Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z));
    }

    /// Switch the rendered primitive.
    /// Allowed: "triangle", "cube", "cylinder", "sphere", "torus".
    pub fn set_primitive(&mut self, name: &str) {
        if let Some(p) = Primitive::from_str(name) {
            let mesh = make_primitive(p);
            self.renderer.set_mesh(&mesh);
            self.bounds = mesh.bounds;
            self.fit_to_view();
        }
    }

    pub fn fit_to_view(&mut self) {
        let aspect = self.width as f32 / self.height as f32;
        self.camera.fit_to_bounds(self.bounds, aspect);
    }

    /// Rotate/orbit in radians.
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.camera.orbit(delta_yaw, delta_pitch);
    }

    /// Pan in world units (relative to current view).
    pub fn pan(&mut self, right: f32, up: f32) {
        self.camera.pan(right, up);
    }

    /// Zoom factor ( >1 out, <1 in ).
    pub fn zoom(&mut self, factor: f32) {
        self.camera.zoom(factor);
    }

    pub fn draw(&self) {
        let aspect = self.width as f32 / self.height as f32;
        let proj = Mat4::perspective(self.camera.fovy, aspect, self.camera.znear, self.camera.zfar);
        let view = Mat4::look_at(self.camera.eye(), self.camera.target, self.camera.view_up());
        let model = Mat4::identity();
        self.renderer
            .draw(self.width, self.height, &proj.m, &view.m, &model.m);
    }
}

