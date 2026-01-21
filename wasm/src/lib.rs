use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into()?;

    // Vertex shader
    let vs = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec3 position;
        void main() {
            gl_Position = vec4(position, 1.0);
        }
        "#,
    )?;

    // Fragment shader
    let fs = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        void main() {
            gl_FragColor = vec4(0.2, 0.7, 1.0, 1.0);
        }
        "#,
    )?;

    let program = link_program(&gl, &vs, &fs)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
    ];

    let buffer = gl.create_buffer().ok_or("failed buffer")?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let pos = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(pos, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(pos);

    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(())
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    ty: u32,
    src: &str,
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(ty).ok_or("create shader")?;
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_default())
    }
}

fn link_program(
    gl: &WebGlRenderingContext,
    vs: &WebGlShader,
    fs: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("create program")?;
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
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}
