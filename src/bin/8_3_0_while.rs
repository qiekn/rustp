#![allow(unused)]
fn reason() {
  // 将 `optional` 定为 `Option<i32>` 类型
  // let optional = Some(7);
  let optional: Option<i32> = None;

  match optional {
    Some(i) => {
      println!("This is a really long string and `{:?}`", i);
    }
    _ => println!("this is None"),
    // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
  };
}

fn elegant() {
  // 全部都是 `Option<i32>` 类型
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
  // 语句块（`{}`）
  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  // 如果要指明失败情形，就使用 else：
  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else {
    // 解构失败。切换到失败情形。
    println!("Didn't match a number. Let's go with a letter!");
  };

  // 提供另一种失败情况下的条件。
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter!");
  } else {
    // 条件的值为 false。于是以下是默认的分支：
    println!("I don't like letters. Let's go with an emoticon :)!");
  };
}

fn main() {
  reason();
  elegant();
}
