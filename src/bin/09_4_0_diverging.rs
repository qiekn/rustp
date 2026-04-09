use std::fmt::Display;

// ----------------------------------------------------------------------------: basic

struct Point {
  x: f64,
  y: f64,
}

impl Point {
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    ((x1 - x2) * (y1 - y2)).abs()
  }

  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

impl Display for Rectangle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "rect: p1({}, {}), p2({}, {})",
      self.p1.x, self.p1.y, self.p2.x, self.p2.y
    )
  }
}

// ----------------------------------------------------------------------------: pair

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
  fn destroy(self) {
    let Pair(first, second) = self;
    println!("Destorying Pair({}, {})", first, second);
  }
}

fn main() {
  let rect = Rectangle {
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  println!("Rectangle area: {}", rect.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  println!("square: {}", square);
  square.translate(1.0, 1.0);
  println!("square: {}", square);

  let pair = Pair(Box::new(1), Box::new(2));
  println!("Pair: {:?}", pair);
  pair.destroy();

  // println!("Pair: {:?}", pair);
}
