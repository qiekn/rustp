fn main() {
  fn function(i: i32) -> i32 {
    i + 1
  }

  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i| i + 1;

  // 译注：将闭包绑定到引用的说法可能不准。
  // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
  // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
  // `closure_xxx` 变量解引用。

  let i = 1;
  // 调用函数和闭包。
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  let one = || 1;
  println!("closure returning one: {}", one());
}
