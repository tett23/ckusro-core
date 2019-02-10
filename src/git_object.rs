use git2::ObjectType;
use std::fmt;

use super::compressed_git_object::{CompressedGitObject, CompressedGitObjectError};

#[derive(Debug, PartialEq)]
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

  pub fn from_u8_vec(data: &Vec<u8>) -> Result<GitObject, CompressedGitObjectError> {
    let compressed = CompressedGitObject::new(data);
    let (object_type, length, content) = compressed.parse()?;

    Ok(GitObject::new(object_type, length, &content))
  }
}

impl fmt::Display for GitObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {}", self.object_type, self.length)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use flate2::write::ZlibEncoder;
  use flate2::Compression;
  use std::io::prelude::*;

  fn deflate(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();

    encoder.finish().unwrap()
  }

  #[test]
  fn git_object_from_u8_vec() {
    let bytes = deflate(b"blob 1\0a");
    let actual = GitObject::from_u8_vec(&bytes);
    let expected = GitObject::new(ObjectType::Blob, 1, &b"a".to_vec());

    assert_eq!(actual, Ok(expected))
  }
}
