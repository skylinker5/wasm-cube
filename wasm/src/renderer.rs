use wasm_bindgen::prelude::*;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation,
};

use crate::shader::{compile_shader, link_program, FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};

const TRIANGLE_VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

pub(crate) struct Renderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    position_location: u32,
    mvp_location: WebGlUniformLocation,
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

        let buffer = gl
            .create_buffer()
            .ok_or_else(|| js_error("failed to create buffer"))?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        upload_f32_slice(
            &gl,
            WebGlRenderingContext::ARRAY_BUFFER,
            &TRIANGLE_VERTICES,
            WebGlRenderingContext::STATIC_DRAW,
        );

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

        let mvp_location = gl
            .get_uniform_location(&program, "u_mvp")
            .ok_or_else(|| js_error("missing uniform u_mvp"))?;

        Ok(Self {
            gl,
            program,
            buffer,
            position_location,
            mvp_location,
        })
    }

    pub(crate) fn draw_triangle(&self, width: i32, height: i32, mvp: &[f32; 16]) {
        // Keep `program`/`buffer` fields alive; WebGL resources are tied to JS GC.
        let _ = (
            &self.program,
            &self.buffer,
            self.position_location,
            &self.mvp_location,
        );

        self.gl.viewport(0, 0, width, height);
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(&self.mvp_location), false, mvp);

        self.gl
            .clear_color(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }
}

fn upload_f32_slice(gl: &WebGlRenderingContext, target: u32, data: &[f32], usage: u32) {
    unsafe {
        let view = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(target, &view, usage);
    }
}

fn js_error(msg: &str) -> JsValue {
    JsValue::from_str(msg)
}
