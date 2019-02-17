use super::error::Error;
use git2;
use git2::{Object, ObjectType, Odb, OdbObject, Oid};
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

  pub fn find_object(&self, oid: Oid) -> Result<OdbObject, git2::Error> {
    self.odb.read(oid)
  }

  pub fn add_object(&self, kind: ObjectType, data: &[u8]) -> Result<Oid, git2::Error> {
    self.odb.write(kind, data)
  }

  pub fn exists(&self, oid: Oid) -> bool {
    self.odb.exists(oid)
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

  fn test_add_object() {
    let repo = BundledRepository::new().unwrap();
    let oid = repo.add_object(ObjectType::Blob, b"test file").unwrap();
    let actual = repo.exists(oid);

    assert!(actual)
  }
}
