use failure::Fail;
use std::fmt;

// mod super::compressed_git_object;
use super::compressed_git_object::CompressedGitObject;

#[derive(Debug, PartialEq)]
pub enum ObjectTypes {
  Blob,
}

impl ObjectTypes {
  pub fn from_str(name: &str) -> Result<ObjectTypes, ObjectTypesError> {
    match name {
      "blob" => Ok(ObjectTypes::Blob),
      _ => Err(ObjectTypesError::InvalidTypeName {
        name: name.to_owned(),
      }),
    }
  }
}

#[derive(PartialEq, Debug, Fail)]
pub enum ObjectTypesError {
  #[fail(display = "invalid type name. name = {}", name)]
  InvalidTypeName { name: String },
}

impl fmt::Display for ObjectTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
pub struct GitObject {
  object_type: ObjectTypes,
  length: u64,
  content: Vec<u8>,
}

impl GitObject {
  pub fn new(object_type: ObjectTypes, length: u64, content: &Vec<u8>) -> GitObject {
    GitObject {
      object_type,
      length,
      content: content.to_owned(),
    }
  }

  pub fn from_compressed_git_object(compressed: &CompressedGitObject) -> GitObject {
    let (object_type, length, content) = compressed.parse().unwrap();

    GitObject::new(object_type, length, &content)
  }

  pub fn parse_object(&self) {
    println!("{}", self.length)
  }
}

impl fmt::Display for GitObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.length)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    // let obj = GitObject::new(&"".as_bytes().to_vec());

    // println!("{}", obj)
  }
}
