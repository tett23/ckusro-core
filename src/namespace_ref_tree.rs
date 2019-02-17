use super::namespace::Namespace;
use super::namespace_ref::NamespaceRef;
use super::tree::{Leaf, Node, Tree};

pub struct NamespaceRefTree(Tree<NamespaceRef>);

impl NamespaceRefTree {
  pub fn new() -> NamespaceRefTree {
    NamespaceRefTree(Tree::Node(Node::new(None)))
  }

  // pub fn find(&self, namespace: &Namespace) -> Option<&Tree<NamespaceRef>> {
  //   self.0.find(namespace);
  //   let ns_ref = match self.0 {
  //     Tree::Leaf(v) => panic!("{}", v.value().namespace().name()),
  //     Tree::Node(v) => v,
  //   };

  //   return None;
  //   //     let item =self.0.iter().find(|item| match item {
  //   //       Tree::Leaf(v) => v.name() === node.name() && v.namespace_type() == node.namespace_type(),
  //   //       Tree::Node(v, _) => v.name() === node.name() && v.namespace_type() == node.namespace_type()
  //   //     });

  //   // item
  // }
}
