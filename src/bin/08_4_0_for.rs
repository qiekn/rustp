#![allow(unreachable_code)]

fn use_label_to_break_outer_loop() {
  'outer: loop {
    println!("Entered the outer loop");

    loop {
      println!("Entered the inner loop");

      // 这只是中断内部的循环
      // 无法中断外层 loop
      // break;

      // 这会中断外层循环
      break 'outer;
    }

    println!("This point will never be reached");
  }

  println!("Exited the outer loop");
}

fn main() {
  use_label_to_break_outer_loop();

  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  assert_eq!(result, 20);
}
