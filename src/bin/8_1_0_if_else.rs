fn for_in_range() {
  // [1, 11)
  for n in 1..11 {
    println!("{}", n);
  }

  println!("---------------------");

  // [1, 11]
  for n in 1..=11 {
    println!("{}", n);
  }
}

fn for_iterator() {
  let names = vec!["j8blow", "bob", "qiken"];

  // test 1 normal for each
  for name in names.iter() {
    match name {
      &"j8blow" => println!("{} is ....", name),
      _ => println!("hello, {}", name),
    }
  }

  // test 2: mut

  let mut names = vec!["j8blow", "bob", "qiken"];
  for name in names.iter_mut() {
    *name = match name {
      &mut "j8blow" => "{} is ....",
      _ => "hello, {}",
    }
  }

  // test 3: moved
  for name in names.into_iter() {
    match name {
      "Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
}

fn main() {
  for_iterator();
  for_in_range();
}
