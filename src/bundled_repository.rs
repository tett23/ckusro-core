use super::error::Error;
use git2::Odb;
use std::fmt;

pub struct BundledRepository<'a> {
  odb: Odb<'a>,
}

impl<'a> BundledRepository<'a> {
  pub fn new() -> Result<BundledRepository<'a>, Error> {
    let odb: Odb<'a> = match Odb::new() {
      Ok(v) => Ok(v),
      Err(err) => Err(Error::OdbInitializationFailed {
        detail: err.message().to_owned(),
      }),
    }?;

    let ret = BundledRepository { odb };

    Ok(ret)
  }
}

impl<'a> fmt::Display for BundledRepository<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "BundledRepository")
  }
}

impl<'a> fmt::Debug for BundledRepository<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "BundledRepository")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bundled_repository_new() {
    let actual = BundledRepository::new();

    assert!(actual.is_ok());
  }
}
