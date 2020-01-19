mod gl_util;
mod node;
mod scene;

use crate::scene::Scene;
use wasm_bindgen::prelude::*;

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
  fn clearInterval(token: f64);
  fn alert(msg: &str);
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
    Interval { closure, token }
  }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
  fn drop(&mut self) {
    clearInterval(self.token);
  }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  let elements = include_str!("input.csv").lines();
  let scene = Scene::new(elements, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
  scene.draw_scene();
  // Interval::new(1_00, move || {
  //   scene.draw_scene();
  // });
  Ok(())
}
