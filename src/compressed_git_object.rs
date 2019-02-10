extern crate flate2;
use super::error::Error;
use flate2::read::ZlibDecoder;
use git2::ObjectType;
use std::io::prelude::*;

#[derive(Debug)]
pub struct CompressedGitObject<'a> {
  content: &'a Vec<u8>,
}

impl<'a> CompressedGitObject<'a> {
  pub fn new(content: &'a Vec<u8>) -> CompressedGitObject {
    CompressedGitObject { content }
  }

  pub fn parse(&self) -> Result<(ObjectType, u64, Vec<u8>), Error> {
    let inflated = inflate(self.content)?;
    let (header, content) = split_object(&inflated)?;
    let (object_type, length) = parse_header(&header)?;

    Ok((object_type, length, content))
  }
}

fn inflate(value: &Vec<u8>) -> Result<Vec<u8>, Error> {
  let value: &[u8] = value;
  let mut d = ZlibDecoder::new(value);
  let mut ret: Vec<u8> = Vec::new();

  match d.read_to_end(&mut ret) {
    Ok(_) => Ok(ret),
    Err(_) => Err(Error::InvalidZlibData),
  }
}

fn split_object(value: &Vec<u8>) -> Result<(String, Vec<u8>), Error> {
  let pos = find_null_pos(&value)?;
  let (header, content) = value.split_at(pos);
  let content = content[1..].to_vec();
  let header = match String::from_utf8(header.to_vec()) {
    Ok(v) => Ok(v),
    Err(_) => Err(Error::EncodingError),
  }?;

  Ok((header, content))
}

fn parse_header(header: &str) -> Result<(ObjectType, u64), Error> {
  let mut header = header.split_whitespace();

  let object_type = match header.next() {
    Some(v) => Ok(v),
    None => Err(Error::InvalidHeader),
  }?;
  let object_type = to_object_type(object_type)?;

  let length = match header.next() {
    Some(v) => Ok(v),
    None => Err(Error::InvalidHeader),
  }?;
  let length = match length.parse() {
    Ok(v) => Ok(v),
    Err(_) => Err(Error::InvalidHeader),
  }?;

  Ok((object_type, length))
}

fn to_object_type(name: &str) -> Result<ObjectType, Error> {
  match name {
    "blob" => Ok(ObjectType::Blob),
    "tree" => Ok(ObjectType::Tree),
    "commit" => Ok(ObjectType::Commit),
    "tag" => Ok(ObjectType::Tag),
    _ => Err(Error::InvalidTypeName),
  }
}

fn find_null_pos(content: &Vec<u8>) -> Result<usize, Error> {
  const NULL: u8 = 0;
  let pos = content.into_iter().position(|&v| v == NULL);

  match pos {
    Some(pos) => Ok(pos),
    _ => Err(Error::NullCharacterNotFound),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use flate2::write::ZlibEncoder;
  use flate2::Compression;

  fn deflate(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();

    encoder.finish().unwrap()
  }

  #[test]
  fn test_parse() {
    let bytes = deflate(b"blob 1\0a");
    let actual = CompressedGitObject::new(&bytes).parse();
    let expected = (ObjectType::Blob, 1, "a".as_bytes().to_vec());

    assert_eq!(actual.unwrap(), expected)
  }

  #[test]
  fn test_inflate() {
    let bytes = deflate(b"foo");
    let actual = inflate(&bytes);
    let expected = b"foo".to_vec();

    assert_eq!(actual, Ok(expected))
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

    assert_eq!(actual.unwrap_err(), Error::NullCharacterNotFound);
  }

  #[test]
  fn test_parse_header() {
    let header = "blob 1";
    let actual = parse_header(&header).unwrap();

    assert_eq!(actual.0, ObjectType::Blob);
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

  #[test]
  fn test_to_object_type() {
    let actual = to_object_type(&"blob");

    assert_eq!(actual, Ok(ObjectType::Blob));
  }

  #[test]
  fn test_to_object_type_when_passed_invalid_type_name() {
    let actual = to_object_type(&"invalid_type_name");

    assert_eq!(actual, Err(Error::InvalidTypeName));
  }
}
