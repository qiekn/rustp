// 🦀 See: https://rustwiki.org/zh-CN/rust-by-example/hello/print.html
fn main() {
  println!("today is {}", "Thursday");

  // 26 is i32, if you want int64, use 26i64
  println!("today is {} {}, {}", "Mar", 26, 2026);

  println!("my name is {0}, I'm {}", "qiekn");

  // 命名参数
  println!("my name is {author_name}", author_name = "qiekn");

  // 可以在 `:` 后面指定特殊的格式。
  println!("{} of {:b} people know binary, the other half don't", 1, 2);

  println!(
    "{} of {:x} people understand hex, the rest are confused",
    1, 16
  );
  println!("{} of {:o} people understand octal", 1, 8);

  // 你可以按指定宽度来右对齐文本。
  // 下面语句输出 "     1"，5 个空格后面连着 1。
  println!("{number:>width$}", number = 1, width = 6);

  // 你可以在数字左边补 0。下面语句输出 "000001"。
  println!("{number:>0width$}", number = 1, width = 6);

  println!("test: {:🦀>10}", 123);

  let pi = 3.141592;
  println!("Pi is roughly {:.*}", 3, pi);
}
