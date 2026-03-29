use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
  println!("first element of the slice: {}", slice[0]);
  println!("the slice has {} elements", slice.len());

  for value in slice {
    print!("{}, ", value);
  }
  println!("\n");
}

fn main() {
  // 定长数组（类型标记是多余的）
  // int[5] this_is_a_array
  let mut this_is_a_array: [i32; 5] = [1, 2, 3, 4, 5];

  // 所有元素可以初始化成相同的值
  let yet_another_array: [i32; 500] = [-1; 500];

  // 下标从 0 开始
  println!("first element of the array: {}", this_is_a_array[0]); // 1
  println!("second element of the array: {}", this_is_a_array[1]); // 2

  // `len` 返回数组的大小
  println!("array size: {}", this_is_a_array.len());

  this_is_a_array.reverse();
  println!(
    "first element of the reversed array: {}",
    this_is_a_array[0]
  ); // 5

  // 数组是在栈中分配的
  println!(
    "array occupies {} bytes",
    mem::size_of_val(&this_is_a_array)
  );

  // 数组可以自动被借用成为 slice
  println!("borrow the whole array as a slice");
  analyze_slice(&this_is_a_array);

  // slice 可以指向数组的一部分
  println!("borrow a section of the array as a slice");
  analyze_slice(&this_is_a_array[1..4]);

  // 越界的下标会引发致命错误（panic）
  // println!("{}", this_is_a_array[5]);
}
