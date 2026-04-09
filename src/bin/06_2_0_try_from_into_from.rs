use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

#[derive(Debug, PartialEq)]
struct NotEvenNumber(i32);

impl Display for NotEvenNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} is not an even number (by qiekn)", self.0)
  }
}

impl TryFrom<i32> for EvenNumber {
  type Error = NotEvenNumber;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value % 2 == 0 {
      Ok(EvenNumber(value))
    } else {
      Err(NotEvenNumber(value))
    }
  }
}

fn main() {
  // TryFrom

  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(5), Err(NotEvenNumber(5)));

  let num = EvenNumber::try_from(13);

  match num {
    Ok(n) => println!("success: {:?}", n),
    Err(e) => println!("error: {}", e),
  }

  // TryInto

  let result: Result<EvenNumber, NotEvenNumber> = 8i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));
  let result: Result<EvenNumber, NotEvenNumber> = 5i32.try_into();
  assert_eq!(result, Err(NotEvenNumber(5)));
}
