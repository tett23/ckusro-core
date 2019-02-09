use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ObjectTypes {
  Blob,
}

impl Display for ObjectTypes {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "{}",
      match self {
        ObjectTypes::Blob => "Blob",
      }
    )
  }
}

#[derive(Debug)]
pub struct GitObject<'a> {
  object_type: ObjectTypes,
  value: &'a [u8],
  size: u64,
}

impl<'a> GitObject<'a> {
  pub fn new(value: &'a [u8]) -> GitObject {
    let obj = GitObject {
      object_type: ObjectTypes::Blob,
      value,
      size: 0,
    };

    obj.parse_object();

    obj
  }

  pub fn parse_object(&self) {
    println!("{}", self.size)
  }
}

impl<'a> Display for GitObject<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.size)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let obj = GitObject::new("".as_bytes());

    println!("{}", obj)
  }

  #[test]
  fn it_works2() {
    let obj = GitObject::new("".as_bytes());

    obj.parse_object()
  }
}
