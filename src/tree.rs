#[derive(Debug, PartialEq)]
pub struct Leaf<T: PartialEq>(Box<T>);

impl<T: PartialEq> Leaf<T> {
  pub fn new(v: T) -> Leaf<T> {
    Leaf(Box::new(v))
  }
}

#[derive(Debug, PartialEq)]
pub struct Node<T: PartialEq + Debug>(Box<Option<T>>, Vec<Tree<T>>);

impl<T: PartialEq + Debug> Node<T> {
  pub fn new(v: Option<T>) -> Node<T> {
    Node(Box::new(v), Vec::new())
  }

  pub fn push(&mut self, v: Tree<T>) {
    self.1.push(v)
  }

  pub fn find(&self, value: &T) -> Option<&Tree<T>> {
    self
      .1
      .iter()
      .find(|item| match (*item).find_by_value(value) {
        Some(_) => true,
        None => false,
      })
  }

  pub fn children<'a>(&'a self) -> Vec<&'a Tree<T>> {
    let ret = self
      .1
      .iter()
      .map(|item| {
        let item: &'a Tree<T> = item;

        item
      })
      .collect();

    return ret;
  }
}

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Tree<T: PartialEq + Debug> {
  Leaf(Leaf<T>),
  Node(Node<T>),
}

impl<T: PartialEq + Debug> Tree<T> {
  fn compare_node(&self, other: &Tree<T>) -> bool {
    match (self, other) {
      (Tree::Leaf(Leaf(v1)), Tree::Leaf(Leaf(v2))) => v1 == v2,
      (Tree::Node(_), Tree::Leaf(_)) => false,
      (Tree::Leaf(_), Tree::Node(_)) => false,
      (Tree::Node(Node(v1, _)), Tree::Node(Node(v2, _))) => v1 == v2,
    }
  }

  pub fn find<'a>(&'a self, value: &'a Tree<T>) -> Option<&'a Tree<T>> {
    let ret = match self {
      Tree::Leaf(_) => match self == value {
        true => Some(self),
        false => None,
      },
      Tree::Node(node) => match self == value {
        true => Some(self),
        false => Tree::_find(&node.children(), value),
      },
    };

    ret
  }

  fn _find<'a>(vec: &Vec<&'a Tree<T>>, value: &Tree<T>) -> Option<&'a Tree<T>> {
    let pos = vec
      .into_iter()
      .position(|item| match item.compare_node(value) {
        true => true,
        false => match item {
          Tree::Leaf(_) => false,
          Tree::Node(node) => Tree::_find(&node.children(), value).is_some(),
        },
      });

    let ret = match pos {
      None => None,
      Some(pos) => {
        let item: &'a Tree<T> = &*vec[pos];
        match item.compare_node(value) {
          true => Some(item),
          false => match item {
            Tree::Leaf(_) => Some(item),
            Tree::Node(node) => {
              let vec = node.children();
              Tree::_find(&vec, value)
            }
          },
        }
      }
    };

    return ret;
  }

  pub fn find_by_value(&self, value: &T) -> Option<&Tree<T>> {
    match self {
      Tree::Leaf(Leaf(tag)) => match *value == **tag {
        true => Some(self),
        false => None,
      },
      Tree::Node(node) => match node {
        Node(tag, _) => match tag.as_ref() {
          Some(tag) => match value == tag {
            true => Some(self),
            false => match node.find(value) {
              Some(node) => Some(node),
              None => None,
            },
          },
          None => node.find(value),
        },
      },
    }
  }

  // pub fn parent(&self, value: &Tree<T>) -> Option<&Tree<T>> {
  //   match self {
  //     Tree::Leaf(_) => None,
  //     Tree::Node(node) => match node.find(value).is_some() {
  //       true => Some(self),
  //       false => match node
  //         .children()
  //         .iter()
  //         .find(|item| item.parent(value).is_some())
  //       {
  //         Some(v) => Some(v),
  //         None => None,
  //       },
  //     },
  //   }
  // }

  // pub fn pwd(&self, tree: Tree<T>) -> &str {
  //   tree
  // }
}

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::*;

  fn fill_tree() -> Box<Tree<Namespace>> {
    let repo1 = Leaf(Box::new(Namespace::new(
      NamespaceType::Repository,
      "ckusro-core",
    )));

    let user_ns = Some(Namespace::new(NamespaceType::User, "tett23"));
    let mut user_node = Node::new(user_ns);
    user_node.push(Tree::Leaf(repo1));

    let domain_ns = Some(Namespace::new(NamespaceType::Domain, "github.com"));
    let mut domain_node = Node::new(domain_ns);
    domain_node.push(Tree::Node(user_node));

    let mut root_node = Node::new(None);
    root_node.push(Tree::Node(domain_node));

    Box::new(Tree::Node(root_node))
  }

  fn print_tree(tree: Box<Tree<Namespace>>) {
    match *tree {
      Tree::Node(node) => {
        let ns = node.0;
        let vec = node.1;
        let ns_name = match *ns {
          Some(ns) => ns.name(),
          None => "root".to_owned(),
        };
        println!("node {}", ns_name);
        for a in vec {
          print_tree(Box::new(a))
        }
      }
      Tree::Leaf(ns) => {
        let ns = ns.0;
        let ns_name = &*ns.name();
        println!("leaf {}", ns_name)
      }
    }
  }

  #[test]
  fn test_tree() {
    let root = *fill_tree();
    print_tree(Box::new(root));
  }

  mod tree {
    use super::*;

    #[test]
    fn test_find() {
      let root = *fill_tree();

      let ns = Tree::Node(Node::new(Some(Namespace::new(
        NamespaceType::Domain,
        "github.com",
      ))));
      let actual = root.find(&ns);
      assert!(actual.is_some());

      let ns = Tree::Node(Node::new(Some(Namespace::new(
        NamespaceType::User,
        "tett23",
      ))));
      let actual = root.find(&ns);
      assert!(actual.is_some());

      let ns = Tree::Leaf(Leaf::new(Namespace::new(
        NamespaceType::Repository,
        "ckusro-core",
      )));
      let actual = root.find(&ns);
      assert!(actual.is_some());
    }

    #[test]
    fn test_find_by_value() {
      let root = *fill_tree();
      let actual = root.find_by_value(&Namespace::new(NamespaceType::Domain, "github.com"));

      assert!(actual.is_some());
    }
  }
}
