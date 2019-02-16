enum Tree<T> {
  Leaf(Box<T>),
  Node(Box<Option<T>>, Vec<Tree<T>>),
}

impl<T> Tree<T> {
  pub fn new_node(b: Box<Option<T>>) -> Tree<T> {
    let vec: Vec<Tree<T>> = Vec::new();

    Tree::Node(b, vec)
  }

  pub fn new_leaf(v: T) -> Tree<T> {
    Tree::Leaf(Box::new(v))
  }

  pub fn push(&mut self, v: Tree<T>) {
    match self {
      Tree::Leaf(_) => panic!(""),
      Tree::Node(_, vec) => vec.push(v),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::*;

  fn get_node(b: Box<Option<Namespace>>) -> Box<Tree<Namespace>> {
    Box::new(Tree::new_node(b))
  }

  fn fill_tree() -> Box<Tree<Namespace>> {
    let repo1 = Tree::Leaf(Box::new(Namespace::new(
      NamespaceType::Repository,
      "ckusro",
    )));

    let user_ns = Box::new(Some(Namespace::new(NamespaceType::User, "tett23")));
    let mut user_node = *get_node(user_ns);
    user_node.push(repo1);

    let domain_ns = Box::new(Some(Namespace::new(NamespaceType::Domain, "github.com")));
    let mut domain_node = *get_node(domain_ns);
    domain_node.push(user_node);

    let mut root_node = *get_node(Box::new(None));
    root_node.push(domain_node);

    Box::new(root_node)
  }

  fn print_tree(tree: Box<Tree<Namespace>>) {
    match *tree {
      Tree::Node(ns, vec) => {
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
        let ns_name = &*ns.name();
        println!("leaf {}", ns_name)
      }
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
