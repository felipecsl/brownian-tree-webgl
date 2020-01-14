mod gl_util;

use crate::gl_util::compile_and_link_program;
use crate::gl_util::get_context;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

const VERTEX_SHADER_SOURCE: &'static str = r#"
  attribute vec2 a_position;
  uniform vec2 u_resolution;
  uniform vec2 u_translation;

  void main() {
    // Add in the translation.
    vec2 position = a_position + u_translation;
    // convert the position from pixels to 0.0 to 1.0
    vec2 zeroToOne = position / u_resolution;
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

#[wasm_bindgen]
extern "C" {
  fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
  fn cancelInterval(token: f64);
}

#[wasm_bindgen]
pub struct Interval {
  closure: Closure<dyn FnMut()>,
  token: f64,
}

impl Interval {
  pub fn new<F: 'static>(millis: u32, f: F) -> Interval
  where
    F: FnMut(),
  {
    let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
    // Pass the closuer to JS, to run every n milliseconds.
    let token = setInterval(&closure, millis);
    Interval {
      closure,
      token
    }
  }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
  fn drop(&mut self) {
      cancelInterval(self.token);
  }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  let gl = get_context()?;
  let program = compile_and_link_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
  let canvas = gl.canvas().unwrap().dyn_into::<HtmlCanvasElement>()?;
  let position_attribute_location = gl.get_attrib_location(&program, &"a_position") as u32;
  let translation_uniform_location = gl.get_uniform_location(&program, &"u_translation");
  let resolution_uniform_location = gl.get_uniform_location(&program, &"u_resolution");
  let color_uniform_location = gl.get_uniform_location(&program, "u_color");
  let buffer = gl.create_buffer().unwrap();
  gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
  set_geometry(&gl, 5, 5);
  draw_scene(
    &gl,
    &canvas,
    &program,
    position_attribute_location,
    translation_uniform_location.as_ref(),
    resolution_uniform_location.as_ref(),
    color_uniform_location.as_ref(),
  );
  Interval::new(1_00, move || {
    draw_scene(
      &gl,
      &canvas,
      &program,
      position_attribute_location,
      translation_uniform_location.as_ref(),
      resolution_uniform_location.as_ref(),
      color_uniform_location.as_ref(),
    );
  });
  Ok(())
}

fn draw_scene(
  gl: &web_sys::WebGlRenderingContext,
  canvas: &HtmlCanvasElement,
  program: &WebGlProgram,
  position_attribute_location: u32,
  translation_uniform_location: Option<&WebGlUniformLocation>,
  resolution_uniform_location: Option<&WebGlUniformLocation>,
  color_uniform_location: Option<&WebGlUniformLocation>,
) {
  gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
  gl.use_program(Some(program));
  gl.enable_vertex_attrib_array(position_attribute_location);
  gl.vertex_attrib_pointer_with_i32(
    position_attribute_location,
    2,
    WebGlRenderingContext::FLOAT,
    false,
    0,
    0,
  );
  gl.uniform2f(
    resolution_uniform_location,
    canvas.width() as f32,
    canvas.height() as f32,
  );
  let mut rng = rand::thread_rng();
  for _ in 0..100 {
    gl.uniform4f(
      color_uniform_location,
      rng.gen(),
      rng.gen(),
      rng.gen(),
      1.0,
    );
    let translation = [
      rng.gen_range(0.0, canvas.width() as f32),
      rng.gen_range(0.0, canvas.height() as f32),
    ];
    gl.uniform2fv_with_f32_array(translation_uniform_location, &translation);
    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
  }
}

fn set_geometry(gl: &web_sys::WebGlRenderingContext, width: usize, height: usize) {
  let x1: f32 = 0.0 as f32;
  let x2: f32 = width as f32;
  let y1: f32 = 0.0 as f32;
  let y2: f32 = height as f32;
  // Note that `Float32Array::view` is somewhat dangerous (hence the
  // `unsafe`!). This is creating a raw view into our module's
  // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
  // (aka do a memory allocation in Rust) it'll cause the buffer to change,
  // causing the `Float32Array` to be invalid.
  //
  // As a result, after `Float32Array::view` we have to be very careful not to
  // do any memory allocations before it's dropped.
  unsafe {
    gl.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &js_sys::Float32Array::view(&[x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]),
      WebGlRenderingContext::STATIC_DRAW,
    );
  }
}
