extern crate failure;
extern crate flate2;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;

#[derive(Debug)]
pub struct CompressedGitObject<'a> {
  content: &'a Vec<u8>,
}

impl<'a> CompressedGitObject<'a> {
  pub fn new(content: &'a Vec<u8>) -> CompressedGitObject {
    CompressedGitObject { content }
  }
}

fn inflate<'a>(value: &'a Vec<u8>) -> Vec<u8> {
  let value: &[u8] = value;
  let mut d = ZlibDecoder::new(value);
  let mut ret: Vec<u8> = Vec::new();

  d.read_to_end(&mut ret).unwrap();

  ret
}

// fn split_content(value: Vec<u8>) -> (String, Vec<u8>){

// }
// extern crate io;
// use io;
// use std::io;
use failure::Fail;

#[derive(PartialEq, Debug, Fail)]
enum CompressedGitObjectError {
  #[fail(display = "Null character not found.")]
  NullCharacterNotFound,
}

fn find_null_pos(content: &Vec<u8>) -> Result<usize, CompressedGitObjectError> {
  const NULL: u8 = 0;
  let pos = content.binary_search(&NULL);

  // Ok(pos)
  match pos {
    Ok(pos) => Ok(pos),
    _ => Err(CompressedGitObjectError::NullCharacterNotFound),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use flate2::write::ZlibEncoder;
  use flate2::Compression;

  #[test]
  fn test_inflate() {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(b"foo").unwrap();
    let compressed_bytes = encoder.finish().unwrap();
    let ret = inflate(&compressed_bytes);

    assert_eq!(ret, "foo".as_bytes());
  }

  #[test]
  fn test_null_pos() {
    let content = vec![1, 0, 2];
    let actual = find_null_pos(&content);
    let expected: usize = 1;

    assert_eq!(actual.unwrap(), expected);
  }

  #[test]
  fn test_null_pos_when_arg_does_not_include_null_character() {
    let content = vec![1, 2, 3];
    let actual = find_null_pos(&content);

    assert_eq!(
      actual.unwrap_err(),
      CompressedGitObjectError::NullCharacterNotFound
    );
  }
}
