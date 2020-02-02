use crate::gl_util::compile_and_link_program;
use crate::gl_util::get_context;
use crate::gl_util::set_geometry;
use crate::node::parse_node;
use crate::scarlet::colormap::ColorMap;
use crate::Node;
use scarlet::color::RGBColor;
use scarlet::colormap::ListedColorMap;
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
  pub fn new(vertex_shader: &str, fragment_shader: &str) -> Scene {
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
    let nodes = Scene::parse_items(include_str!("input.csv"));
    // let nodes = Scene::gen_items(500);
    // since the input data uses world coordinates (-inf, +inf), we need to place elements
    // at the (x,y) at the center of screen and translate them according to position
    set_geometry(
      &gl,
      (canvas.width() / 2) as f32,
      (canvas.height() / 2) as f32,
      1,
      1,
    );
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

  // fn gen_items(total: u32) -> Vec<Node> {
  //   let mut ptr: *mut Node = unsafe { add_particles(total as i32) };
  //   let mut ret: Vec<Node> = vec![];
  //   let end_rounded_up = ptr.wrapping_offset(total as isize);
  //   while ptr != end_rounded_up {
  //     unsafe { ret.push(*ptr) };
  //     ptr = ptr.wrapping_offset(1);
  //   }
  //   return ret;
  // }

  fn parse_items(csv_source: &str) -> Vec<Node> {
    let mut items = csv_source.lines();
    let mut nodes: Vec<Node> = vec![];
    while let Some(item) = items.next() {
      if !item.is_empty() {
        match parse_node(item) {
          Ok(node) => nodes.push(node),
          Err(e) => println!("Failed to parse node {}", e),
        }
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
    let z_coords = self.nodes.iter().map(|n| n.z).collect::<Vec<f64>>();
    let rgb_coords = z_coords_to_rgb(&z_coords);
    for (i, node) in self.nodes.iter().enumerate() {
      gl.uniform4f(
        self.color_uniform.as_ref(),
        rgb_coords[i].r as f32,
        rgb_coords[i].g as f32,
        rgb_coords[i].b as f32,
        1.0,
      );
      let translation = [node.x as f32, node.y as f32];
      gl.uniform2fv_with_f32_array(self.translation_uniform.as_ref(), &translation);
      gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }

    fn vec_min<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
      *v.iter().min_by(|&x, y| x.partial_cmp(y).unwrap()).unwrap()
    }

    fn vec_max<T: PartialOrd + Copy>(v: &Vec<T>) -> T {
      *v.iter().max_by(|&x, y| x.partial_cmp(y).unwrap()).unwrap()
    }

    fn z_coords_to_rgb(z_coords: &Vec<f64>) -> Vec<RGBColor> {
      let min_z = vec_min(&z_coords);
      let max_z = vec_max(&z_coords);
      // (min: -3, max: 7) -> range: 10
      let z_range = (max_z - min_z) as i32;
      let colormap = ListedColorMap::viridis();
      // limit Z to the range [0..max_z] -> [0..1]
      let normalized_coords = z_coords
        .iter()
        .map(|z| (z - min_z) / z_range as f64)
        .collect::<Vec<f64>>();
      colormap.transform(normalized_coords)
    }
  }
}
