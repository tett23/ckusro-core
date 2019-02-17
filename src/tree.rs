#[derive(Debug, PartialEq)]
pub struct Leaf<T: PartialEq>(Box<T>);

impl<T: PartialEq> Leaf<T> {
  pub fn new(v: T) -> Leaf<T> {
    Leaf(Box::new(v))
  }
}

#[derive(Debug, PartialEq)]
pub struct Node<T: PartialEq>(Box<Option<T>>, Vec<Tree<T>>);

impl<T: PartialEq> Node<T> {
  pub fn new(v: Option<T>) -> Node<T> {
    Node(Box::new(v), Vec::new())
  }

  pub fn push(&mut self, v: Tree<T>) {
    self.1.push(v)
  }

  pub fn find(&self, value: &T) -> Option<&Tree<T>> {
    self.1.iter().find(|item| match (*item).find(value) {
      Some(_) => true,
      None => false,
    })
  }
}

#[derive(Debug, PartialEq)]
pub enum Tree<T: PartialEq> {
  Leaf(Leaf<T>),
  Node(Node<T>),
}

impl<T: PartialEq> Tree<T> {
  pub fn find(&self, value: &T) -> Option<&Tree<T>> {
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
              None => {
                println!("hogehoge");
                None
              }
            },
          },
          None => node.find(value),
        },
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::*;

  fn fill_tree() -> Box<Tree<Namespace>> {
    let repo1 = Leaf(Box::new(Namespace::new(
      NamespaceType::Repository,
      "ckusro",
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
    use super::super::*;
    use super::*;

    #[test]
    fn test_find() {
      let ns_ref = &Namespace::new(NamespaceType::Domain, "github.com");
      let ns = Namespace::new(NamespaceType::Domain, "github.com");
      assert_eq!(ns_ref, &ns);

      let root = *fill_tree();
      let actual = root.find(&Namespace::new(NamespaceType::Domain, "github.com"));

      println!("Tree::find {:?}", actual);
      assert!(actual.is_some());
      // match actual {
      //   Some(v) => v.
      // }
      // assert!(actual.unwrap());
    }
  }
}
