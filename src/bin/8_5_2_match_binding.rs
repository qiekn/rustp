fn age() -> u32 {
  11
}

fn test1() {
  println!("Tell me what type of person you are");

  match age() {
    0 => println!("I haven't celebrated my first birthday yet"),
    // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
    // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
    age @ 1..=12 => println!("I'm a child of age {:?}", age),
    age @ 13..=19 => println!("I'm a teen of age {:?}", age),
    // 不符合上面的范围。返回结果。
    age => println!("I'm an old person of age {:?}", age),
  }
}

fn some_number() -> Option<u32> {
  // Some(42)
  None
}

fn test2() {
  match some_number() {
    // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
    Some(42) => println!("The Answer: {}!", 42),
    // 匹配任意其他数字。
    Some(n) => println!("Not interesting... {}", n),
    // 匹配任意其他值（`None` 可变类型）。
    _ => println!("none"),
  }
}

fn main() {
  test1();
  test2();
}
