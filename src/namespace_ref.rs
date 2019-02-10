use super::namespace::{Namespace, NamespaceType};
use git2::Oid;

pub struct NamespaceRef {
  namespace: Namespace,
  oid: Oid,
}

impl NamespaceRef {
  pub fn new(namespace: Namespace, oid: Oid) -> NamespaceRef {
    NamespaceRef { namespace, oid }
  }
}

// pub trait NamespaceParent {
//   fn parent(&self) -> Namespace;
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_namespace_ref_new() {
//     let namespace = Namespace::new(NamespaceType::User, "tett23");
//     let oid = Oid::from_str("test_commit").unwrap();
//     let actual = NamespaceRef::new(namespace, oid);

//     assert!(actual.is_ok());
//   }
// }
