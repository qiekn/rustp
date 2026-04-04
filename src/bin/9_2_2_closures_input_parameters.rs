// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F)
where
  F: Fn(),
{
  f();
}

fn main() {
  let x = 7;

  let print_var = || println!("{}", x);

  apply(print_var);
}
