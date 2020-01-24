use crate::Node;
use core::num::ParseFloatError;
use core::num::ParseIntError;
use std::str::FromStr;

/** Parse a node object from a CSV line */
pub fn parse_node(line: &str) -> Result<Node, String> {
  let parts: Vec<&str> = line.split(',').collect();
  fn int_err(err: ParseIntError) -> String {
    format!("failed to parse int in: {}", err)
  }
  fn float_err(err: ParseFloatError) -> String {
    format!("failed to parse float in: {}", err)
  }
  Ok(Node {
    id: i32::from_str_radix(parts[0], 10).map_err(int_err)?,
    parent_id: i32::from_str_radix(parts[1], 10).map_err(int_err)?,
    x: f64::from_str(parts[2]).map_err(float_err)?,
    y: f64::from_str(parts[3]).map_err(float_err)?,
    z: f64::from_str(parts[4]).map_err(float_err)?,
  })
}
