use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub(crate) const VERTEX_SHADER_SRC: &str = r#"
attribute vec3 position;
uniform mat4 u_mvp;
void main() {
    gl_Position = u_mvp * vec4(position, 1.0);
}
"#;

pub(crate) const FRAGMENT_SHADER_SRC: &str = r#"
void main() {
    gl_FragColor = vec4(0.2, 0.7, 1.0, 1.0);
}
"#;

pub(crate) fn compile_shader(
    gl: &WebGlRenderingContext,
    ty: u32,
    src: &str,
) -> Result<WebGlShader, JsValue> {
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

pub(crate) fn link_program(
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
