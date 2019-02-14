use super::error::Error;
use super::namespace::Namespace;
use super::namespace_ref::NamespaceRef;
use std::rc::Rc;

pub struct NamespaceRefManager<'a> {
  namepspace_refs: Vec<Rc<&'a NamespaceRef<'a>>>,
}

impl<'a> NamespaceRefManager<'a> {
  pub fn new() -> NamespaceRefManager<'a> {
    NamespaceRefManager {
      namepspace_refs: Vec::new(),
    }
  }

  // pub fn add_namespace_ref(&mut self, ns_ref: Rc<&'a NamespaceRef<'a>>) -> Option<Error> {
  //   self.namepspace_refs.push(ns_ref);

  //   match &ns_ref.parent {
  //     Some(parent) => self.add_namespace_ref(Rc::new(parent)),
  //     None => None,
  //   }?;

  //   None
  // }
}

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::super::path_fragment::PathFragment;
  use super::*;
  use git2::Oid;

  // fn fragment_refs_fixture(
  //   fragment: &str,
  // ) -> (
  //   Box<&NamespaceRef>,
  //   (
  //     Box<&PathFragment>,
  //     Box<&Namespace>,
  //     Box<&NamespaceRef>,
  //     Box<&Namespace>,
  //     Box<&NamespaceRef>,
  //     Box<&Namespace>,
  //   ),
  // ) {
  //   let fragment = match PathFragment::parse_full_qualified_fragment(fragment) {
  //     Ok(v) => v,
  //     Err(_) => PathFragment {
  //       domain: "github.com".to_owned(),
  //       user: "tett23".to_owned(),
  //       repository: "ckusro-core".to_owned(),
  //     },
  //   };

  //   let domain_ns = Namespace::new(NamespaceType::Domain, &fragment.domain);
  //   let domain_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //   let domain_ref = NamespaceRef::new(domain_ns, domain_oid, None);

  //   let user_ns = Namespace::new(NamespaceType::User, &fragment.user);
  //   let user_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //   let user_ref = NamespaceRef::new(user_ns, user_oid, Some(Rc::new(&domain_ref)));

  //   let repo_ns = Namespace::new(NamespaceType::Repository, &fragment.repository);
  //   let repo_oid = Oid::from_bytes(b"12345678901234567890").unwrap();
  //   let repo_ref = NamespaceRef::new(repo_ns, repo_oid, Some(Rc::new(&user_ref)));

  //   (
  //     Box::new(&repo_ref),
  //     (
  //       Box::new(&fragment),
  //       Box::new(&domain_ns),
  //       Box::new(&domain_ref),
  //       Box::new(&user_ns),
  //       Box::new(&user_ref),
  //       Box::new(&repo_ns),
  //     ),
  //   )
  // }

  mod namespace_ref_manager {
    use super::*;

    mod add_namespace_ref {
      use super::*;

      // #[test]
      // fn test_add_namespace_ref() {
      //   let mut manager = NamespaceRefManager::new();
      //   let (repo_ref, _) = fragment_refs_fixture("github.com@tett23:ckusro-core");

      //   manager.add_namespace_ref(Rc::new(&repo_ref));
      // }
    }
  }
}
