extern crate failure;

use failure::Fail;

#[derive(PartialEq, Debug, Fail)]
pub enum Error {
  #[fail(display = "Null character not found.")]
  InvalidZlibData,
  #[fail(display = "Null character not found.")]
  NullCharacterNotFound,
  #[fail(display = "Encoding error.")]
  EncodingError,
  #[fail(display = "Invalid header.")]
  InvalidHeader,
  #[fail(display = "Invalid type name.")]
  InvalidTypeName,
  #[fail(display = "Odb initialization failed. detail: {}", detail)]
  OdbInitializationFailed { detail: String },
  #[fail(display = "{} is not match {}", t1, t2)]
  NamespaceMismatch { t1: String, t2: String },
  #[fail(display = "Invalid path fragment. fragment={}", _0)]
  MalformedFragment(String),
}
