type Ll = u64;
type Qiekn = u64;

// 通过这个属性屏蔽警告。
#[allow(non_camel_case_types)]
type u64_t = u64;
// 试一试 ^ 移除上面那个属性

fn main() {
  let a: Ll = 5 as u64_t;
  let b: Qiekn = 2 as u64_t;

  // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
  println!(
    "{} nanoseconds + {} inches = {} unit?",
    a,
    b,
    a + b
  );
}
