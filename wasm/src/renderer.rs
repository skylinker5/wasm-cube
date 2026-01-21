use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

const VERTEX_SHADER_SRC: &str = r#"
attribute vec3 position;
void main() {
    gl_Position = vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
void main() {
    gl_FragColor = vec4(0.2, 0.7, 1.0, 1.0);
}
"#;

const TRIANGLE_VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

pub(crate) struct Renderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    position_location: u32,
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

        Ok(Self {
            gl,
            program,
            buffer,
            position_location,
        })
    }

    pub(crate) fn draw_triangle(&self) {
        // Keep `program`/`buffer` fields alive; WebGL resources are tied to JS GC.
        let _ = (&self.program, &self.buffer, self.position_location);

        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }
}

fn upload_f32_slice(gl: &WebGlRenderingContext, target: u32, data: &[f32], usage: u32) {
    unsafe {
        let view = js_sys::Float32Array::view(data);
        gl.buffer_data_with_array_buffer_view(target, &view, usage);
    }
}

fn compile_shader(gl: &WebGlRenderingContext, ty: u32, src: &str) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(ty)
        .ok_or_else(|| js_error("failed to create shader"))?;
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(js_error(&gl.get_shader_info_log(&shader).unwrap_or_else(
            || "unknown shader compilation error".to_string(),
        )))
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vs: &WebGlShader,
    fs: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl
        .create_program()
        .ok_or_else(|| js_error("failed to create program"))?;
    gl.attach_shader(&program, vs);
    gl.attach_shader(&program, fs);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(js_error(&gl.get_program_info_log(&program).unwrap_or_else(
            || "unknown program link error".to_string(),
        )))
    }
}

fn js_error(msg: &str) -> JsValue {
    JsValue::from_str(msg)
}
