mod gl_util;

use crate::gl_util::bind_buffer;
use crate::gl_util::compile_and_link_program;
use crate::gl_util::get_context;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext;

const VERTEX_SHADER_SOURCE: &'static str = r#"
  attribute vec2 a_position;
  uniform vec2 u_resolution;

  void main() {
    // convert the position from pixels to 0.0 to 1.0
    vec2 zeroToOne = a_position / u_resolution;
    // convert from 0->1 to 0->2
    vec2 zeroToTwo = zeroToOne * 2.0;
    // convert from 0->2 to -1->+1 (clip space)
    vec2 clipSpace = zeroToTwo - 1.0;
    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
  }
"#;
const FRAGMENT_SHADER_SOURCE: &'static str = r#"
  precision mediump float;
  uniform vec4 u_color;

  void main() {
    gl_FragColor = u_color;
  }
"#;

const VERTICES: [f32; 12] = [
  10.0, 20.0, 80.0, 20.0, 10.0, 50.0, 10.0, 50.0, 80.0, 20.0, 80.0, 50.0,
];

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  let gl = get_context()?;
  let program = compile_and_link_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
  let canvas = gl.canvas().unwrap().dyn_into::<HtmlCanvasElement>()?;
  let position_attribute_location = gl.get_attrib_location(&program, &"a_position") as u32;
  let resolution_uniform_location = gl.get_uniform_location(&program, &"u_resolution");
  let color_uniform_location = gl.get_uniform_location(&program, "u_color");
  let buffer = bind_buffer(&gl, &VERTICES);
  gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
  gl.use_program(Some(&program));
  gl.enable_vertex_attrib_array(position_attribute_location);
  gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
  gl.vertex_attrib_pointer_with_i32(
    position_attribute_location,
    2,
    WebGlRenderingContext::FLOAT,
    false,
    0,
    0,
  );
  gl.uniform2f(
    resolution_uniform_location.as_ref(),
    canvas.width() as f32,
    canvas.height() as f32,
  );
  gl.uniform4f(color_uniform_location.as_ref(), 1.0, 0.0, 0.0, 1.0);
  gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
  Ok(())
}
