use std::fmt::Display;

struct Persion {
  sex: &'static str,
  age: i32,
  weight: f64,
}

impl Display for Persion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let title = if self.age < 30 {
      "这是一个初阶魔法师"
    } else {
      "这是一个大魔法师"
    };

    write!(f, "{}", title)
  }
}

fn main() {
  let qiekn = Persion {
    sex: "male",
    age: 18,
    weight: 10000000000.0,
  };

  println!("QIEKN: {}", qiekn);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
