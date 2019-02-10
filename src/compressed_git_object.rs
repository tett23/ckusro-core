extern crate failure;
extern crate flate2;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;

use super::git_object::ObjectTypes;

#[derive(Debug)]
pub struct CompressedGitObject<'a> {
  content: &'a Vec<u8>,
}

impl<'a> CompressedGitObject<'a> {
  pub fn new(content: &'a Vec<u8>) -> CompressedGitObject {
    CompressedGitObject { content }
  }

  pub fn parse(&self) -> Result<(ObjectTypes, u64, Vec<u8>), CompressedGitObjectError> {
    let inflated = inflate(self.content);
    let (header, content) = split_object(&inflated)?;
    let (object_type, length) = parse_header(&header)?;

    Ok((object_type, length, content))
  }
}

fn inflate<'a>(value: &'a Vec<u8>) -> Vec<u8> {
  let value: &[u8] = value;
  let mut d = ZlibDecoder::new(value);
  let mut ret: Vec<u8> = Vec::new();

  d.read_to_end(&mut ret).unwrap();

  ret
}

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

fn parse_header(header: &str) -> Result<(ObjectTypes, u64), CompressedGitObjectError> {
  let mut header = header.split_whitespace();

  let object_type = match header.next() {
    Some(v) => Ok(v),
    None => Err(CompressedGitObjectError::InvalidHeader),
  }?;
  let object_type = match ObjectTypes::from_str(object_type) {
    Ok(v) => Ok(v),
    Err(_) => Err(CompressedGitObjectError::InvalidHeader),
  }?;

  let length = match header.next() {
    Some(v) => Ok(v),
    None => Err(CompressedGitObjectError::InvalidHeader),
  }?;
  let length = match length.parse() {
    Ok(v) => Ok(v),
    Err(_) => Err(CompressedGitObjectError::InvalidHeader),
  }?;

  Ok((object_type, length))
}

use failure::Fail;

#[derive(PartialEq, Debug, Fail)]
pub enum CompressedGitObjectError {
  #[fail(display = "Null character not found.")]
  NullCharacterNotFound,
  #[fail(display = "Encoding error.")]
  EncodingError,
  #[fail(display = "Invalid header.")]
  InvalidHeader,
}

fn find_null_pos(content: &Vec<u8>) -> Result<usize, CompressedGitObjectError> {
  const NULL: u8 = 0;
  let pos = content.into_iter().position(|&v| v == NULL);

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
  fn test_parse() {
    let bytes = Vec::from(b"blob 1\0a".to_vec());
    let encoder = ZlibEncoder::new(bytes, Compression::default());
    let compressed_bytes = encoder.finish().unwrap();

    let actual = CompressedGitObject::new(&compressed_bytes).parse();
    let expected = (ObjectTypes::Blob, 1, "a".as_bytes().to_vec());

    assert_eq!(actual.unwrap(), expected)
  }

  #[test]
  fn test_inflate() {
    let bytes = Vec::from(b"foo".to_vec());
    let encoder = ZlibEncoder::new(bytes, Compression::default());
    let compressed_bytes = encoder.finish().unwrap();
    let ret = inflate(&compressed_bytes);

    assert_eq!(ret, b"foo")
  }

  #[test]
  fn test_split_object() {
    let actual = split_object(&b"blob 1\0a".to_vec());
    let expected_header = "blob 1".to_owned();
    let expected_content = b"a".to_vec();
    let expected = (expected_header, expected_content);

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

    assert_eq!(actual.0, ObjectTypes::Blob);
    assert_eq!(actual.1, 1);
  }

  #[test]
  fn test_parse_header_when_arg_does_not_contain_spaces() {
    let header = "blob1";
    let actual = parse_header(&header);

    assert!(actual.is_err());
  }
  //
  #[test]
  fn test_parse_header_when_arg_does_not_contain_2_more_spaces() {
    let header = "blob hoge 1";
    let actual = parse_header(&header);

    assert!(actual.is_err());
  }
}
