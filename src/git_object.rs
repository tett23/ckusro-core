use git2::ObjectType;
use std::fmt;

use super::compressed_git_object::CompressedGitObject;

#[derive(Debug)]
pub struct GitObject {
  object_type: ObjectType,
  length: u64,
  content: Vec<u8>,
}

impl GitObject {
  pub fn new(object_type: ObjectType, length: u64, content: &Vec<u8>) -> GitObject {
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
