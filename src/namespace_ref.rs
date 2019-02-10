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
