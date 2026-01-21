use wasm_bindgen::prelude::*;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation,
};

use crate::geometry::Mesh;
use crate::shader::{compile_shader, link_program, FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};

pub(crate) struct Renderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    vbo: WebGlBuffer,
    nbo: WebGlBuffer,
    ibo: Option<WebGlBuffer>,
    position_location: u32,
    normal_location: u32,
    model_location: WebGlUniformLocation,
    view_location: WebGlUniformLocation,
    proj_location: WebGlUniformLocation,
    light_dir_location: WebGlUniformLocation,
    index_count: i32,
    vertex_count: i32,
}

impl Renderer {
    pub(crate) fn new(gl: WebGlRenderingContext) -> Result<Self, JsValue> {
        let vs = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
        let fs = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SRC,
        )?;

        let program = link_program(&gl, &vs, &fs)?;
        gl.use_program(Some(&program));

        gl.enable(WebGlRenderingContext::DEPTH_TEST);

        let vbo = gl
            .create_buffer()
            .ok_or_else(|| js_error("failed to create position buffer"))?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vbo));

        let position_location = gl.get_attrib_location(&program, "position") as u32;
        gl.vertex_attrib_pointer_with_i32(
            position_location,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(position_location);

        let nbo = gl
            .create_buffer()
            .ok_or_else(|| js_error("failed to create normal buffer"))?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&nbo));
        let normal_location = gl.get_attrib_location(&program, "normal") as u32;
        gl.vertex_attrib_pointer_with_i32(
            normal_location,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(normal_location);

        let model_location = gl
            .get_uniform_location(&program, "u_model")
            .ok_or_else(|| js_error("missing uniform u_model"))?;
        let view_location = gl
            .get_uniform_location(&program, "u_view")
            .ok_or_else(|| js_error("missing uniform u_view"))?;
        let proj_location = gl
            .get_uniform_location(&program, "u_proj")
            .ok_or_else(|| js_error("missing uniform u_proj"))?;
        let light_dir_location = gl
            .get_uniform_location(&program, "u_light_dir_vs")
            .ok_or_else(|| js_error("missing uniform u_light_dir_vs"))?;

        Ok(Self {
            gl,
            program,
            vbo,
            nbo,
            ibo: None,
            position_location,
            normal_location,
            model_location,
            view_location,
            proj_location,
            light_dir_location,
            index_count: 0,
            vertex_count: 0,
        })
    }

    pub(crate) fn set_mesh(&mut self, mesh: &Mesh) {
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vbo));
        upload_f32_slice(
            &self.gl,
            WebGlRenderingContext::ARRAY_BUFFER,
            &mesh.positions,
            WebGlRenderingContext::STATIC_DRAW,
        );

        self.vertex_count = (mesh.positions.len() / 3) as i32;

        // Upload normals.
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.nbo));
        upload_f32_slice(
            &self.gl,
            WebGlRenderingContext::ARRAY_BUFFER,
            &mesh.normals,
            WebGlRenderingContext::STATIC_DRAW,
        );

        if mesh.indices.is_empty() {
            self.ibo = None;
            self.index_count = 0;
            return;
        }

        let ibo = self
            .gl
            .create_buffer()
            .ok_or_else(|| js_error("failed to create index buffer"))
            .unwrap();
        self.gl
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        upload_u16_slice(
            &self.gl,
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &mesh.indices,
            WebGlRenderingContext::STATIC_DRAW,
        );
        self.ibo = Some(ibo);
        self.index_count = mesh.indices.len() as i32;
    }

    pub(crate) fn draw(
        &self,
        width: i32,
        height: i32,
        proj: &[f32; 16],
        view: &[f32; 16],
        model: &[f32; 16],
    ) {
        // Keep `program`/buffers fields alive; WebGL resources are tied to JS GC.
        let _ = (
            &self.program,
            &self.vbo,
            &self.nbo,
            &self.ibo,
            self.position_location,
            self.normal_location,
            &self.model_location,
            &self.view_location,
            &self.proj_location,
            &self.light_dir_location,
        );

        self.gl.viewport(0, 0, width, height);
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&self.model_location), false, model);
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&self.view_location), false, view);
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&self.proj_location), false, proj);
        // Light pointing from camera toward the scene with slight tilt.
        self.gl.uniform3f(Some(&self.light_dir_location), -0.3, -0.5, -1.0);

        self.gl
            .clear_color(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        // Ensure buffers are bound at draw time.
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vbo));
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.nbo));
        if let Some(ibo) = &self.ibo {
            self.gl.bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(ibo),
            );
            self.gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                self.index_count,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        } else {
            self.gl.draw_arrays(
                WebGlRenderingContext::TRIANGLES,
                0,
                self.vertex_count.max(0),
            );
        }
    }
}

fn upload_f32_slice(gl: &WebGlRenderingContext, target: u32, data: &[f32], usage: u32) {
    unsafe {
        let view = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(target, &view, usage);
    }
}

fn upload_u16_slice(gl: &WebGlRenderingContext, target: u32, data: &[u16], usage: u32) {
    unsafe {
        let view = js_sys::Uint16Array::view(data);
        gl.buffer_data_with_array_buffer_view(target, &view, usage);
    }
}

fn js_error(msg: &str) -> JsValue {
    JsValue::from_str(msg)
}
