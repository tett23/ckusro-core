// enum Tree<T> {
//   Leaf(T),
//   Node(Box<Tree<T>>, Box<Tree<T>>),
// }

enum Tree<T> {
  // Leaf(T),
  Leaf(Box<T>),
  // Node(Box<Vec<Box<Tree<T>>>>),
  Node(Vec<Tree<T>>),
}

impl<T> Tree<T> {
  pub fn new_node() -> Tree<T> {
    let vec: Vec<Tree<T>> = Vec::new();

    Tree::Node(vec)
  }

  pub fn new_leaf(v: T) -> Tree<T> {
    Tree::Leaf(Box::new(v))
  }
}

// use extern crate tree;

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::*;

  fn get_node() -> Box<Tree<Namespace>> {
    Box::new(Tree::new_node())
  }

  fn fill_tree() -> Box<Tree<Namespace>> {
    let repo1 = Tree::Leaf(Box::new(Namespace::new(
      NamespaceType::Repository,
      "ckusro",
    )));

    let mut user1 = match *get_node() {
      Tree::Node(v) => v,
      Tree::Leaf(_) => panic!(""),
    };
    user1.push(repo1);

    let mut domain1 = match *get_node() {
      Tree::Node(v) => v,
      Tree::Leaf(_) => panic!(""),
    };
    domain1.push(Tree::Node(user1));

    let mut root = match *get_node() {
      Tree::Node(v) => v,
      Tree::Leaf(_) => panic!(""),
    };

    root.push(Tree::Node(domain1));

    Box::new(Tree::Node(root))
  }

  fn print_tree(tree: Box<Tree<Namespace>>) {
    match *tree {
      Tree::Node(v) => {
        println!("node {}", v.len());
        for a in v {
          print_tree(Box::new(a))
        }
      }
      Tree::Leaf(v) => println!("leaf {:?}", v),
    }
  }

  #[test]
  fn test_tree() {
    let root = *fill_tree();
    // let mut root = match root {
    //   Tree::Node(v) => v,
    //   Tree::Leaf(_) => panic!(""),
    // };

    // root.push(*get_node());

    // print_tree(Box::new(Tree::Node(root)));
    print_tree(Box::new(root));

    // println!("{:?}", tree);
    // // let vec: Box<Vec<Tree<Namespace>>> = Box::new(Vec::new());
    // let vec: Vec<Tree<Box<Namespace>>> = Vec::new();
    // let mut tree = Tree::Node(vec);
    // let leaf1 = Tree::Leaf(Box::new(Namespace::new(NamespaceType::User, "tett23")));
    // let leaf2 = Tree::Leaf(Box::new(Namespace::new(NamespaceType::User, "test_user")));
    // vec.push(leaf1);
    // let mut mt = tree.0;

    // // *tree.push(node2);
  }
}
