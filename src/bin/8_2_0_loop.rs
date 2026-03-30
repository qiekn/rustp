fn set_to_2(y: &mut i32) -> bool {
  *y = 2;
  true
}

fn main() {
  let x = 1;

  if x > 0 {
    println!("{} is postive", x);
  } else if x < 0 {
    println!("{} is negtive", x);
  } else {
    println!("is zero");
  }

  let mut y = 1;
  if y != 1 && set_to_2(&mut y) {}
  println!("y:{}", y);
}
