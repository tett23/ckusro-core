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

// fn parse_object(value: Vec<u8>) -> Result<(&str, u64, Vec<u8>), CompressedGitObjectError> {
//   let (header, content) = split_object(&value)?;
//   let (object_type, length) = parse_header(&header)?;

//   Ok((object_type, length, content))
// }

fn split_object(value: &Vec<u8>) -> Result<(String, Vec<u8>), CompressedGitObjectError> {
  let pos = find_null_pos(&value)?;
  let (header, content) = value.split_at(pos);
  let content = content[1..].to_vec();
  let header = match String::from_utf8(header.to_vec()) {
    Ok(v) => Ok(v),
    Err(_) => Err(CompressedGitObjectError::EncodingError),
  }?;

  Ok((header, content))
}

fn parse_header(header: &str) -> Result<(&str, u64), CompressedGitObjectError> {
  let mut header = header.split_whitespace();

  let object_type = match header.next() {
    Some(v) => Ok(v),
    None => Err(CompressedGitObjectError::InvalidHeader),
  }?;
  let length = match header.next() {
    Some(v) => Ok(v),
    None => Err(CompressedGitObjectError::InvalidHeader),
  }?;
  let length = match length.parse() {
    Ok(v) => Ok(v),
    Err(_) => Err(CompressedGitObjectError::InvalidHeader),
  }?;

  let ret = (object_type, length);

  Ok(ret)
}

use failure::Fail;

#[derive(PartialEq, Debug, Fail)]
enum CompressedGitObjectError {
  #[fail(display = "Null character not found.")]
  NullCharacterNotFound,
  #[fail(display = "Encoding error.")]
  EncodingError,
  #[fail(display = "Invalid header.")]
  InvalidHeader,
}

fn find_null_pos(content: &Vec<u8>) -> Result<usize, CompressedGitObjectError> {
  println!("find null pos {:?}", content);
  const NULL: u8 = 0;
  let pos = content.into_iter().position(|&v| v == NULL);

  // Ok(pos)
  match pos {
    Some(pos) => Ok(pos),
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
  fn test_split_object() {
    println!("{}", &"blob 1\0a");
    let actual = split_object(&"blob 1\0a".as_bytes().to_vec());
    let expected = ("blob 1".to_owned(), "a".as_bytes().to_vec());

    assert_eq!(actual.unwrap(), expected)
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

  #[test]
  fn test_parse_header() {
    let header = "blob 1";
    let actual = parse_header(&header).unwrap();

    assert_eq!(actual.0, "blob");
    assert_eq!(actual.1, 1);
  }

  #[test]
  fn test_parse_header_when_arg_does_not_contain_spaces() {
    let header = "blob1";
    let actual = parse_header(&header);

    assert!(actual.is_err());
  }

  #[test]
  fn test_parse_header_when_arg_does_not_contain_2_more_spaces() {
    let header = "blob hoge 1";
    let actual = parse_header(&header);

    assert!(actual.is_err());
  }
}
