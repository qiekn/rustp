#[derive(Debug)]
struct Structrue(i32);

#[derive(Debug)]
struct Deep(Structrue);

fn main() {
  println!("{:?} months in a year.", 12);

  // Christian Slater is the actor's name
  println!(
    "{1:?} {0:?} is the {actor:?} name",
    "Slater",
    "Christian",
    actor = "actor's"
  );

  // {}   <-- std::fmt::Display
  // {:?} <-- std::fmt::Debug
  println!("Now {:?} will print!", Structrue(3));

  println!("Now {:#?} will print!", Deep(Structrue(7)));
}
