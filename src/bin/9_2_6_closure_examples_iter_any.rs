fn main() {
  use std::mem;
  // ----------------------------------------------------------------------------: &var
  let color = String::from("blue");

  let print_color = || println!("current color is {}", color);

  print_color();
  let _test_color = &color;
  print_color();

  // ----------------------------------------------------------------------------: &mut var
  let mut count = 0;

  let mut increase_count = || {
    count += 1;
    println!("current count is {}", count);
  };

  increase_count(); // 1
  increase_count(); // 2
  increase_count(); // 3

  let _count_reborrowed = &mut count;
  *_count_reborrowed += 1;

  // increase_count();

  // ----------------------------------------------------------------------------: move

  let movable = Box::new(3);

  let consume = || {
    println!("`movable` : {:?}", movable);
    mem::drop(movable);
  };

  consume();
  // consume();
}
