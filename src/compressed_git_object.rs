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
}
