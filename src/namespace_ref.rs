use super::namespace::Namespace;
use git2::Oid;

#[derive(Debug, PartialEq, Clone)]
pub struct NamespaceRef {
  namespace: Box<Namespace>,
  oid: Oid,
}

impl NamespaceRef {
  pub fn new(namespace: Namespace, oid: Oid) -> NamespaceRef {
    NamespaceRef {
      namespace: Box::new(namespace),
      oid,
    }
  }
}
