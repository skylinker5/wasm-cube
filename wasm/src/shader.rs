use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub(crate) const VERTEX_SHADER_SRC: &str = r#"
attribute vec3 position;
attribute vec3 normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

varying vec3 v_normal_vs;

void main() {
    vec4 pos_vs = u_view * u_model * vec4(position, 1.0);
    // Transform normal with the upper-left 3x3 of the model-view matrix.
    v_normal_vs = mat3(u_view * u_model) * normal;
    gl_Position = u_proj * pos_vs;
}
"#;

pub(crate) const FRAGMENT_SHADER_SRC: &str = r#"
precision mediump float;

varying vec3 v_normal_vs;

uniform vec3 u_light_dir_vs; // Direction the light travels, in view space.

void main() {
    vec3 n = -normalize(v_normal_vs);
    float ndl = max(dot(n, -normalize(u_light_dir_vs)), 0.0);
    vec3 base = vec3(0.8, 0.85, 0.95);
    vec3 color = base * (0.15 + 0.85 * ndl);
    gl_FragColor = vec4(color, 1.0);
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
