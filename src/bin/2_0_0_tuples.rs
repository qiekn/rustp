use std::fmt::Display;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (int32, bool) = pair;
  (bool, int32)
}

fn transpose(mat: Matrix) -> Matrix {
  // 0 1 2 3
  // 0 2 1 3
  // Matrix(mat.0, mat.2, mat.1, mat.3)

  let Matrix(a, b, c, d) = mat;
  Matrix(a, c, b, d)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "QIEKN DISPLAY: \n({} {})\n({} {})",
      self.0, self.1, self.2, self.3
    )
  }
}

fn main() {
  let long_tuple = (
    1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
  );

  // 通过元组的下标来访问具体的值
  println!("long tuple first value: {}", long_tuple.0);
  println!("long tuple second value: {}", long_tuple.1);

  // 元组也可以充当元组的元素
  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

  // 元组可以打印
  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // 但很长的元组无法打 (最多 12 个)
  let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  println!("too long tuple: {:?}", too_long_tuple);
  // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

  let pair = (1, true);
  println!("pair is {:?}", pair);
  println!("the reversed pair is {:?}", reverse(pair));

  // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
  let mut qiekn_a = (5i32);
  let qiekn_b = (1i32,);
  qiekn_a = qiekn_b.0;

  println!("qiekn test: {:?}", qiekn_a);

  println!("one element tuple: {:?}", (5u32),);
  println!("just an integer: {:?}", (5u32));

  let tuple = (1, "hello", 4.5, true);

  // 元组可以被解构（deconstruct），从而将值绑定给变量
  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{}", matrix);

  println!("-------------------------------");

  println!("Matrix:\n{}", matrix);
  println!("Transpose Matrix:\n{}", transpose(matrix));
}
