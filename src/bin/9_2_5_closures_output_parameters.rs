fn create_fn() -> impl Fn() {
  let text = String::from("Fn");

  move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
  let text = String::from("FnMut");

  move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
  let text = String::from("FnOnce");

  move || println!("This is a: {}", text)
}

fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  let fn_once = create_fnonce();

  fn_plain();
  fn_mut();
  fn_once();
}
