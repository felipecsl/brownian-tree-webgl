use core::num::ParseFloatError;
use core::num::ParseIntError;
use std::str::FromStr;

pub struct Node {
  id: u16,
  parent_id: u16,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Node {
  pub fn parse(line: &str) -> Result<Node, String> {
    let parts: Vec<&str> = line.split(',').collect();
    fn int_err(err: ParseIntError) -> String {
      format!("failed to parse int in: {}", err)
    }
    fn float_err(err: ParseFloatError) -> String {
      format!("failed to parse float in: {}", err)
    }
    Ok(Node {
      id: u16::from_str_radix(parts[0], 10).map_err(int_err)?,
      parent_id: u16::from_str_radix(parts[1], 10).map_err(int_err)?,
      x: f32::from_str(parts[2]).map_err(float_err)?,
      y: f32::from_str(parts[3]).map_err(float_err)?,
      z: f32::from_str(parts[4]).map_err(float_err)?,
    })
  }
}
