use crate::gl_util::compile_and_link_program;
use crate::gl_util::get_context;
use crate::gl_util::set_geometry;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

pub struct Scene {
  gl: WebGlRenderingContext,
  program: WebGlProgram,
  canvas: HtmlCanvasElement,
  position_attrib_location: u32,
  translation_uniform_location: Option<WebGlUniformLocation>,
  resolution_uniform_location: Option<WebGlUniformLocation>,
  color_uniform_location: Option<WebGlUniformLocation>,
}

impl Scene {
  pub fn new(vertex_shader: &str, fragment_shader: &str) -> Scene {
    let gl = get_context().unwrap();
    let program = compile_and_link_program(&gl, vertex_shader, fragment_shader).unwrap();
    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let position_attrib_location = gl.get_attrib_location(&program, &"a_position") as u32;
    let translation_uniform_location = gl.get_uniform_location(&program, &"u_translation");
    let resolution_uniform_location = gl.get_uniform_location(&program, &"u_resolution");
    let color_uniform_location = gl.get_uniform_location(&program, "u_color");
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    set_geometry(&gl, 3, 3);
    Scene {
      gl,
      program,
      canvas,
      position_attrib_location,
      translation_uniform_location,
      resolution_uniform_location,
      color_uniform_location,
    }
  }

  pub fn draw_scene(&self) {
    self.gl.viewport(
      0,
      0,
      self.canvas.width() as i32,
      self.canvas.height() as i32,
    );
    self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
    self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    self.gl.use_program(Some(&self.program));
    self
      .gl
      .enable_vertex_attrib_array(self.position_attrib_location);
    self.gl.vertex_attrib_pointer_with_i32(
      self.position_attrib_location,
      2,
      WebGlRenderingContext::FLOAT,
      false,
      0,
      0,
    );
    self.gl.uniform2f(
      self.resolution_uniform_location.as_ref(),
      self.canvas.width() as f32,
      self.canvas.height() as f32,
    );
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
      self.gl.uniform4f(
        self.color_uniform_location.as_ref(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        1.0,
      );
      let translation = [
        rng.gen_range(0.0, self.canvas.width() as f32),
        rng.gen_range(0.0, self.canvas.height() as f32),
      ];
      self
        .gl
        .uniform2fv_with_f32_array(self.translation_uniform_location.as_ref(), &translation);
      self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }
  }
}
