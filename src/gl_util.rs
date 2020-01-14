use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlShader;


pub fn get_context() -> Result<web_sys::WebGlRenderingContext, Object> {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  canvas
    .get_context("webgl")?
    .unwrap()
    .dyn_into::<WebGlRenderingContext>()
}

pub fn compile_and_link_program(
  context: &WebGlRenderingContext,
  vertex_shader_source: &str,
  fragment_shader_source: &str,
) -> Result<WebGlProgram, String> {
  let vert_shader = compile_shader(
    &context,
    WebGlRenderingContext::VERTEX_SHADER,
    vertex_shader_source,
  )?;
  let frag_shader = compile_shader(
    &context,
    WebGlRenderingContext::FRAGMENT_SHADER,
    fragment_shader_source,
  )?;
  let program = link_program(&context, &vert_shader, &frag_shader)?;
  context.use_program(Some(&program));
  Ok(program)
}

pub fn compile_shader(
  context: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);
  if context
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);
  if context
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}
