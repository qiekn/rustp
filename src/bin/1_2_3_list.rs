use std::fmt;

struct Qiekn(Vec<i32>);

impl fmt::Display for Qiekn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vector = &self.0;

        write!(f, "QIEKN'S VECTOR [")?;

        for (count, item) in vector.iter().enumerate() {
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let boy = Qiekn(vec![1, 2, 3]);
    println!("{}", boy);
}
