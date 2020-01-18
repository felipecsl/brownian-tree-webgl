use crate::gl_util::compile_and_link_program;
use crate::gl_util::get_context;
use crate::gl_util::set_geometry;
use crate::node::Node;
use std::str::Lines;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlUniformLocation;

pub struct Scene {
  gl: WebGlRenderingContext,
  program: WebGlProgram,
  canvas: HtmlCanvasElement,
  position_attrib: u32,
  translation_uniform: Option<WebGlUniformLocation>,
  resolution_uniform: Option<WebGlUniformLocation>,
  color_uniform: Option<WebGlUniformLocation>,
  nodes: Vec<Node>,
}

impl Scene {
  pub fn new(items: Lines<'static>, vertex_shader: &str, fragment_shader: &str) -> Scene {
    let gl = get_context().unwrap();
    let program = compile_and_link_program(&gl, vertex_shader, fragment_shader).unwrap();
    let canvas = gl
      .canvas()
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();
    let position_attrib = gl.get_attrib_location(&program, &"a_position") as u32;
    let translation_uniform = gl.get_uniform_location(&program, &"u_translation");
    let resolution_uniform = gl.get_uniform_location(&program, &"u_resolution");
    let color_uniform = gl.get_uniform_location(&program, "u_color");
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    let nodes = Scene::parse_items(items);
    set_geometry(&gl, (canvas.width() / 2) as f32, (canvas.height() / 2) as f32, 2, 2);
    Scene {
      gl,
      program,
      canvas,
      position_attrib,
      translation_uniform,
      resolution_uniform,
      color_uniform,
      nodes,
    }
  }

  fn parse_items(mut items: Lines<'static>) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    while let Some(item) = items.next() {
      match Node::parse(item) {
        Ok(node) => nodes.push(node),
        Err(e) => println!  ("Failed to parse node {}", e),
      }
    }
    return nodes;
  }

  pub fn draw_scene(&self) {
    let width = self.canvas.width();
    let height = self.canvas.height();
    let gl = &self.gl;
    gl.viewport(0, 0, width as i32, height as i32);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.use_program(Some(&self.program));
    gl.enable_vertex_attrib_array(self.position_attrib);
    gl.vertex_attrib_pointer_with_i32(
      self.position_attrib,
      2,
      WebGlRenderingContext::FLOAT,
      false,
      0,
      0,
    );
    gl.uniform2f(
      self.resolution_uniform.as_ref(),
      width as f32,
      height as f32,
    );
    for (_, node) in self.nodes.iter().enumerate() {
      gl.uniform4f(self.color_uniform.as_ref(), 1.0, 1.0, 1.0, 1.0);
      let translation = [node.x, node.y];
      gl.uniform2fv_with_f32_array(self.translation_uniform.as_ref(), &translation);
      gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }
  }
}
