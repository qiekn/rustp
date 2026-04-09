use std::{convert::From, fmt::Display};

#[derive(Debug)]
struct Number {
  value: i32,
}

impl Display for Number {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "qiekn: {}", self.value)
  }
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

fn main() {
  let count = 5;

  let num = Number::from(count);
  println!("{}", num);

  let num: Number = count.into();
  println!("{}", num);
}
