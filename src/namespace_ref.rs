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

  // pub fn namespace(&self) -> Box<Namespace> {
  //   Box::new(*self.namespace)
  // }

  pub fn oid(&self) -> Oid {
    self.oid
  }
}
