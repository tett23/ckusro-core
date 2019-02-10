use super::error::Error;
use super::namespace::{Namespace, NamespaceType};
use git2::Oid;

#[derive(Debug, PartialEq)]
pub struct NamespaceRef<'a> {
  namespace: &'a Namespace,
  oid: Oid,
  parent: Option<&'a NamespaceRef<'a>>,
}

impl<'a> NamespaceRef<'a> {
  pub fn new(
    namespace: &'a Namespace,
    oid: Oid,
    parent: Option<&'a NamespaceRef>,
  ) -> NamespaceRef<'a> {
    NamespaceRef {
      namespace,
      oid,
      parent,
    }
  }
}

trait HasParent<'a, T> {
  fn parent(&self) -> T;
}

#[derive(Debug, PartialEq)]
struct DomainNamespaceRef<'a>(&'a NamespaceRef<'a>);

impl<'a> DomainNamespaceRef<'a> {
  fn new(ns_ref: &'a NamespaceRef) -> Result<DomainNamespaceRef<'a>, Error> {
    match ns_ref.namespace.namespace_type {
      NamespaceType::Domain => Ok(DomainNamespaceRef(ns_ref)),
      namespace_type => Err(Error::NamespaceMismatch {
        t1: "".to_owned(),
        t2: namespace_type.to_string(),
      }),
    }
  }
}

#[derive(Debug, PartialEq)]
struct UserNamespaceRef<'a>(&'a NamespaceRef<'a>);

impl<'a> UserNamespaceRef<'a> {
  fn new(ns_ref: &'a NamespaceRef) -> Result<UserNamespaceRef<'a>, Error> {
    match ns_ref.namespace.namespace_type {
      NamespaceType::User => Ok(UserNamespaceRef(ns_ref)),
      namespace_type => Err(Error::NamespaceMismatch {
        t1: "UserNamespaceRef".to_owned(),
        t2: namespace_type.to_string(),
      }),
    }
  }
}

impl<'a> HasParent<'a, DomainNamespaceRef<'a>> for UserNamespaceRef<'a> {
  fn parent(&self) -> DomainNamespaceRef<'a> {
    let ns = self.0;

    let ret: DomainNamespaceRef<'a> = DomainNamespaceRef::new(ns.parent.unwrap()).unwrap();

    ret
  }
}

#[derive(Debug, PartialEq)]
struct RepositoryNamespaceRef<'a>(&'a NamespaceRef<'a>);

impl<'a> RepositoryNamespaceRef<'a> {
  fn new(ns_ref: &'a NamespaceRef) -> Result<RepositoryNamespaceRef<'a>, Error> {
    match ns_ref.namespace.namespace_type {
      NamespaceType::Repository => Ok(RepositoryNamespaceRef(ns_ref)),
      namespace_type => Err(Error::NamespaceMismatch {
        t1: "RepositoryNamespaceRef".to_owned(),
        t2: namespace_type.to_string(),
      }),
    }
  }
}

impl<'a> HasParent<'a, UserNamespaceRef<'a>> for RepositoryNamespaceRef<'a> {
  fn parent(&self) -> UserNamespaceRef<'a> {
    let ns = self.0;

    UserNamespaceRef::new(ns.parent.unwrap()).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod user_namespace_ref {
    use super::*;

    #[test]
    fn test_parent() {
      let domain_ns = Namespace::new(NamespaceType::Domain, "github.com");
      let domain_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
      let domain_ref = NamespaceRef::new(&domain_ns, domain_oid, None);

      let user_ns = Namespace::new(NamespaceType::User, "tett23");
      let user_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
      let user_ref = NamespaceRef::new(&user_ns, user_oid, Some(&domain_ref));

      let user_ns_ref = UserNamespaceRef::new(&user_ref).unwrap();
      let actual = user_ns_ref.parent();

      let expected = DomainNamespaceRef::new(&domain_ref).unwrap();

      assert_eq!(actual, expected)
    }
  }

  mod repository_namespace_ref {
    use super::*;

    #[test]
    fn test_parent() {
      let domain_ns = Namespace::new(NamespaceType::Domain, "github.com");
      let domain_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
      let domain_ref = NamespaceRef::new(&domain_ns, domain_oid, None);

      let user_ns = Namespace::new(NamespaceType::User, "tett23");
      let user_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
      let user_ref = NamespaceRef::new(&user_ns, user_oid, Some(&domain_ref));

      let repo_ns = Namespace::new(NamespaceType::Repository, "ckusro-core");
      let repo_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
      let repo_ref = NamespaceRef::new(&repo_ns, repo_oid, Some(&user_ref));

      let repo_ns_ref = RepositoryNamespaceRef::new(&repo_ref).unwrap();
      let actual = repo_ns_ref.parent();

      let expected = UserNamespaceRef::new(&user_ref).unwrap();

      assert_eq!(actual, expected)
    }
  }
}
