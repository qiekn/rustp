use std::fmt::Display;

struct Circle {
  name: String,
  radius: i32,
}

impl Display for Circle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Circle: name is {}, radius is {}.",
      self.name, self.radius
    )
  }
}

fn main() {
  let c = Circle {
    name: "little_c".to_string(),
    radius: 12,
  };

  println!("{}", c.to_string());

  // FromStr
  let parsed: i32 = "5".parse().expect("i32 parse failed");
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println! {"Sum: {:?}", sum};
}
