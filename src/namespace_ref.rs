use super::error::Error;
use super::namespace::{Namespace, NamespaceType};
use git2::Oid;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct NamespaceRef {
  namespace: Namespace,
  oid: Oid,
  pub parent: Option<Rc<NamespaceRef>>,
}

impl NamespaceRef {
  pub fn new(namespace: Namespace, oid: Oid, parent: Option<Rc<NamespaceRef>>) -> NamespaceRef {
    NamespaceRef {
      namespace,
      oid,
      parent,
    }
  }
}

// trait HasParent<T> {
//   fn parent(&self) -> T;
// }

// #[derive(Debug, PartialEq)]
// struct DomainNamespaceRef(Box<NamespaceRef>);

// impl DomainNamespaceRef {
//   fn new(ns_ref: Box<NamespaceRef>) -> Result<DomainNamespaceRef, Error> {
//     match ns_ref.namespace.namespace_type {
//       NamespaceType::Domain => Ok(DomainNamespaceRef(ns_ref)),
//       namespace_type => Err(Error::NamespaceMismatch {
//         t1: "".to_owned(),
//         t2: namespace_type.to_string(),
//       }),
//     }
//   }
// }

// #[derive(Debug, PartialEq)]
// struct UserNamespaceRef(Box<NamespaceRef>);

// impl UserNamespaceRef {
//   fn new(ns_ref: Box<NamespaceRef>) -> Result<UserNamespaceRef, Error> {
//     match ns_ref.namespace.namespace_type {
//       NamespaceType::User => Ok(UserNamespaceRef(ns_ref)),
//       namespace_type => Err(Error::NamespaceMismatch {
//         t1: "UserNamespaceRef".to_owned(),
//         t2: namespace_type.to_string(),
//       }),
//     }
//   }
// }

// use std::ops::Deref;

// impl HasParent<DomainNamespaceRef> for UserNamespaceRef {
//   fn parent(&self) -> DomainNamespaceRef {
//     let ns = self.0.deref();
//     let parent = ns.parent.unwrap();
//     let parent = *parent.deref();
//     let parent = Box::new(parent);

//     DomainNamespaceRef::new(parent).unwrap()
//   }
// }

// #[derive(Debug, PartialEq)]
// struct RepositoryNamespaceRef(Box<NamespaceRef>);

// impl RepositoryNamespaceRef {
//   fn new(ns_ref: Box<NamespaceRef>) -> Result<RepositoryNamespaceRef, Error> {
//     match ns_ref.namespace.namespace_type {
//       NamespaceType::Repository => Ok(RepositoryNamespaceRef(ns_ref)),
//       namespace_type => Err(Error::NamespaceMismatch {
//         t1: "RepositoryNamespaceRef".to_owned(),
//         t2: namespace_type.to_string(),
//       }),
//     }
//   }
// }

// impl HasParent<UserNamespaceRef> for RepositoryNamespaceRef {
//   fn parent(&self) -> UserNamespaceRef {
//     let ns = self.0;

//     UserNamespaceRef::new(ns.parent.unwrap()).unwrap()
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  // mod user_namespace_ref {
  //   use super::*;

  //   #[test]
  //   fn test_parent() {
  //     let domain_ns = Namespace::new(NamespaceType::Domain, "github.com");
  //     let domain_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //     let domain_ref = NamespaceRef::new(Box::new(domain_ns), domain_oid, None);

  //     let user_ns = Namespace::new(NamespaceType::User, "tett23");
  //     let user_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //     let user_ref = NamespaceRef::new(Box::new(user_ns), user_oid, Some(Box::new(domain_ref)));

  //     let user_ns_ref = UserNamespaceRef::new(Box::new(user_ref)).unwrap();
  //     let actual = user_ns_ref.parent();

  //     let expected = DomainNamespaceRef::new(Box::new(domain_ref)).unwrap();

  //     assert_eq!(actual, expected)
  //   }
  // }

  // mod repository_namespace_ref {
  //   use super::*;

  //   #[test]
  //   fn test_parent() {
  //     let domain_ns = Namespace::new(NamespaceType::Domain, "github.com");
  //     let domain_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //     let domain_ref = NamespaceRef::new(Box::new(domain_ns), domain_oid, None);

  //     let user_ns = Namespace::new(NamespaceType::User, "tett23");
  //     let user_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //     let user_ref = NamespaceRef::new(Box::new(user_ns), user_oid, Some(Box::new(domain_ref)));

  //     let repo_ns = Namespace::new(NamespaceType::Repository, "ckusro-core");
  //     let repo_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //     let repo_ref = NamespaceRef::new(Box::new(repo_ns), repo_oid, Some(Box::new(user_ref)));

  //     let repo_ns_ref = RepositoryNamespaceRef::new(Box::new(repo_ref)).unwrap();
  //     let actual = repo_ns_ref.parent();

  //     let expected = UserNamespaceRef::new(Box::new(user_ref)).unwrap();

  //     assert_eq!(actual, expected)
  //   }
  // }
}
