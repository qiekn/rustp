use std::fmt::{self, write};

struct Structure(i32);

#[derive(Debug)]
struct Vec4f {
  x: f64,
  y: f64,
  z: f64,
  w: f64,
}

impl fmt::Display for Structure {
  fn fmt(&self, object: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(object, "qiekn's {:$>15}", self.0)
  }
}

impl fmt::Display for Vec4f {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "QIEKN's vec4 ({:->3},{:$>3},{:?>3},{:🦀>3})",
      self.x, self.y, self.z, self.w
    )
  }
}

fn main() {
  println!("PRINT: {}", Structure(3));

  println!(
    "{}",
    Vec4f {
      x: 1.0,
      y: 1.0,
      z: 1.0,
      w: 1.0,
    }
  );
}
