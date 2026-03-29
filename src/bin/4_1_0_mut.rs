fn main() {
  let a_binding;

  {
    let x = 2;

    a_binding = x * x; // 4
  }
  println!("a binding: {}", a_binding);

  let another_binding;

  // 只做了声明，没有做初始化
  // println!("another binding: {}", another_binding);

  another_binding = 1;
  println!("another binding: {}", another_binding);
}
