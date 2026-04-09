fn base_test() {
  let number = 13;

  println!("Tell me about {}", number);

  match number {
    1 => println!("One!"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
  }

  let boolean = true;
  // match 也是一个表达式
  let binary = match boolean {
    // match 分支必须覆盖所有可能的值
    false => 0,
    true => 1,
    // 试一试 ^ 将其中一条分支注释掉
  };

  println!("binary: {} -> {}", boolean, binary);
}

fn tuple_test() {
  let nums = (0, -2, 3);

  println!("Tell me about {:?}", nums);
  match nums {
    (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
    (1, ..) => println!("First is `1` and the rest doesn't matter"),
    _ => println!("It doesn't matter what they are"),
  }
}

fn enum_test() {
  #[allow(dead_code)]
  enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
  }
  let color = Color::HSL(122, 17, 40);
  // 试一试 ^ 将不同的值赋给 `color`

  println!("What color is it?");
  // 可以使用 `match` 来解构 `enum`。
  match color {
    Color::Red => println!("The color is Red!"),
    Color::Blue => println!("The color is Blue!"),
    Color::Green => println!("The color is Green!"),
    Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
    Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
    Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
    Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
    Color::CMYK(c, m, y, k) => println!(
      "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
      c, m, y, k
    ),
    // 不需要其它分支，因为所有的情形都已覆盖
  }
}

fn references_test() {
  // 获得一个 `i32` 类型的引用。`&` 表示取引用。
  let reference = &4;

  match reference {
    // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
    // `&i32`（译注：即 `reference` 的类型）
    // `&val`（译注：即用于匹配的模式）
    // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
    // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
    &val => println!("Got a value via destructuring: {:?}", val),
  }

  // 如果不想用 `&`，需要在匹配前解引用。
  match *reference {
    val => println!("Got a value via dereferencing: {:?}", val),
  }

  // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
  // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
  let _not_a_reference = 3;

  // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
  // 下面这行将得到一个引用。
  let ref _is_a_reference = 3;

  // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
  let value = 5;
  let mut mut_value = 6;

  // 使用 `ref` 关键字来创建引用。
  // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
  // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
  // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
  // 引用。
  match value {
    ref r => println!("Got a reference to a value: {:?}", r), // 5
  }

  // 类似地使用 `ref mut`。
  match mut_value {
    ref mut m => {
      // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m); // 16
    }
  }
}

fn struct_test() {
  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  // 解构结构体的成员
  let foo = Foo { x: (1, 2), y: 3 };
  let Foo { x: (a, b), y } = foo;

  println!("a = {}, b = {},  y = {} ", a, b, y);

  // 可以解构结构体并重命名变量，成员顺序并不重要

  let Foo { y: i, x: j } = foo;
  println!("i = {:?}, j = {:?}", i, j);

  // 也可以忽略某些变量
  let Foo { y, .. } = foo;
  println!("y = {}", y);

  // 这将得到一个错误：模式中没有提及 `x` 字段
  // let Foo { y } = foo;
}

fn main() {
  base_test();
  tuple_test();
  enum_test();
  references_test();
}
