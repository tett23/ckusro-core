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
  fn compare_tree(&self, other: &Tree<T>) -> bool {
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
      .position(|item| match item.compare_tree(value) {
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
        match item.compare_tree(value) {
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
  //   if self.compare_tree(value) {
  //     return None;
  //   };

  //   match self {
  //     Tree::Leaf(_) => None,
  //     Tree::Node(root) => {
  //       let is_exist = root
  //         .children()
  //         .iter()
  //         .any(|child| child.compare_tree(value));
  //       if is_exist {
  //         return Some(self);
  //       }

  //       let items = root.children().iter().map(|item| item.children()).flatten();

  //       root.children().iter().map(|child| child.parent(value));

  //       None
  //     }
  //   };
  // }

  // pub fn pwd(&self, tree: Tree<T>) -> &str {
  //   tree
  // }
}

struct EnumerateNode<'a, T: PartialEq + Debug> {
  node: &'a Node<T>,
  count: usize,
  consumed_self: bool,
}

impl<'a, T: PartialEq + Debug> EnumerateNode<'a, T> {
  pub fn new(node: &Node<T>) -> EnumerateNode<T> {
    EnumerateNode {
      node,
      count: 0,
      consumed_self: false,
    }
  }
}

// impl<I> Iterator for EnumerateNode<I>
// where
//   I: Iterator,
// {
//   type Item = (usize, <I as Iterator>::Item);

//   fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
//     self.iter.next().map(|a| {
//       let ret = (self.count, a);
//       // Possible undefined overflow.
//       self.count += 1;
//       ret
//     })
//   }
// }
impl<'a, T: PartialEq + Debug> Iterator for EnumerateNode<'a, T> {
  type Item = &'a Tree<T>;

  fn next(&mut self) -> Option<&'a Tree<T>> {
    // if !self.consumed_self{
    //   self.consumed_self = true;
    //   return Some(&node);
    // }

    match self.node.1.iter().nth(self.count) {
      None => None,
      Some(v) => {
        self.count += 1;

        Some(v)
      }
    }
  }
}

// impl<T: PartialEq + Debug> Iterator for Node<T> {
//   type Item = Tree<T>;

//   fn next(&mut self) -> Option<Tree<T>> {
//     self.1.into_iter().next()
//     // EnumerateNode::new(self).next()

//     // self.children().iter().map(|item| {
//     //   items.push(item);
//     //   // item.children().iter().map(|i| i.);
//     // });

//     // None
//   }
// }

// impl<T: PartialEq + Debug> Iterator for Tree<T> {
//   type Item = Tree<T>;

//   fn next(&self) -> Option<Tree<T>> {
//     match self {
//       Tree::Leaf(_) => None,
//       Tree::Node(node) => node.iter().next(),
//     }
//   }
// }

#[cfg(test)]
mod tests {
  use super::super::namespace::{Namespace, NamespaceType};
  use super::*;

  macro_rules! node {
    ($k:expr) => {
      Tree::Node(Node::new(Some($k)))
    };
    () => {
      Tree::Node(Node::new(None))
    };
  }

  macro_rules! leaf {
    ($v:expr) => {
      Tree::Leaf(Leaf::new($v))
    };
  }

  macro_rules! tree {
    () => {node!()};

    ([
      $($v:expr),*
    ]) => {{
      let mut ret = node!();

      $(
        push_node(&mut ret, $v);
      )*

      ret
    }};

    ($v:expr) => {leaf!($v)};

    (
      $tag:expr,
      [
        $($v:expr),*
      ]
    ) => {{
      let mut ret = node!($tag);

      $(
        push_node(&mut ret, $v);
      )*

      ret
    }};
  }

  fn print_tree<T: PartialEq + Debug>(tree: &Tree<T>, indent: Option<usize>) {
    let depth = match indent {
      None => 0,
      Some(v) => v,
    };
    let whitespace = " ".repeat(depth);

    match tree {
      Tree::Leaf(v) => println!("{}{:?}", whitespace, v.0),
      Tree::Node(v) => {
        println!("{}{:?}: {}", whitespace, v.0, '{');
        v.children()
          .iter()
          .for_each(|vv| print_tree(vv, Some(depth + 2)));
        println!("{}{}", whitespace, '}');
      }
    }
  }

  fn push_node<T: PartialEq + Debug>(node: &mut Tree<T>, value: Tree<T>) {
    match node {
      Tree::Leaf(_) => {}
      Tree::Node(node) => node.push(value),
    }
  }

  fn fill_tree() -> Tree<Namespace> {
    let root = tree!([tree!(
      Namespace::new(NamespaceType::Domain, "github.com"),
      [tree!(
        Namespace::new(NamespaceType::User, "tett23"),
        [tree!(Namespace::new(
          NamespaceType::Repository,
          "ckusro-core"
        ))]
      )]
    )]);

    root
  }

  mod tree {
    use super::*;

    mod compare_tree {
      use super::*;

      #[test]
      fn test_node_and_node() {
        let left = node!("test");
        let right = node!("test");
        assert!(left.compare_tree(&right));

        let left = node!("test");
        let mut right = node!("test");
        push_node(&mut right, node!("foo"));
        assert!(left.compare_tree(&right));

        let left = node!("test");
        let right = node!("hoge");
        assert!(!left.compare_tree(&right));

        let left = node!("test");
        let right = node!();
        assert!(!left.compare_tree(&right));
      }

      #[test]
      fn test_node_and_leaf() {
        let left = node!("test");
        let right = leaf!("test");
        assert!(!left.compare_tree(&right));
      }

      #[test]
      fn test_leaf_and_node() {
        let left = leaf!("test");
        let right = node!("test");
        assert!(!left.compare_tree(&right));
      }

      #[test]
      fn test_leaf_and_leaf() {
        let left = leaf!("test");
        let right = leaf!("test");
        assert!(left.compare_tree(&right));

        let left = leaf!("test");
        let right = leaf!("hoge");
        assert!(!left.compare_tree(&right));
      }
    }

    #[test]
    fn test_find() {
      let root = fill_tree();

      let ns = node!(Namespace::new(NamespaceType::Domain, "github.com"));
      let actual = root.find(&ns);
      assert!(actual.is_some());

      let ns = node!(Namespace::new(NamespaceType::User, "tett23",));
      let actual = root.find(&ns);
      assert!(actual.is_some());

      let ns = leaf!(Namespace::new(NamespaceType::Repository, "ckusro-core"));
      let actual = root.find(&ns);
      assert!(actual.is_some());
    }

    #[test]
    fn test_find_by_value() {
      let root = fill_tree();
      let actual = root.find_by_value(&Namespace::new(NamespaceType::Domain, "github.com"));

      assert!(actual.is_some());
    }
  }

  mod node {
    use super::*;

    struct TreeIterator<'a, T: PartialEq + Debug> {
      root: &'a Tree<T>,
      index: usize,
      consumed_self: bool,
    }

    impl<'a, T: PartialEq + Debug> TreeIterator<'a, T> {
      pub fn new(root: &'a Tree<T>) -> TreeIterator<'a, T> {
        TreeIterator {
          root,
          index: 0,
          consumed_self: false,
        }
      }

      // pub fn empty_iter() -> TreeIterator<'a, T> {
      //   let node: &'a Tree<T> = &Tree::Node(Node::new(None));

      //   TreeIterator {
      //     root: node,
      //     index: 0,
      //     consumed_self: false,
      //   }
      // }
    }

    impl<'a, T: PartialEq + Debug> Iterator for TreeIterator<'a, T> {
      type Item = &'a Tree<T>;

      fn next(&mut self) -> Option<&'a Tree<T>> {
        // println!("next {}", self.index);
        match self.root {
          Tree::Leaf(_) => None,
          Tree::Node(node) => {
            let v = node.1.iter().nth(self.index);
            if v.is_none() {
              return None;
            }

            if !self.consumed_self {
              self.consumed_self = true;
              return v;
            }

            self.index += 1;

            TreeIterator::new(v.unwrap()).next()
          }
        }
      }
    }

    // fn get_iter<'a, T: PartialEq + Debug>(root: &'a Tree<T>) -> TreeIterator<'a, T> {
    //   match root {
    //     Tree::Leaf(_) => TreeIterator::new(&root),
    //     Tree::Node(Node(_, vec)) => {
    //       if vec.iter().count() == 0 {
    //         return TreeIterator::empty_iter();
    //       }
    //       if vec.iter().count() == 1 {
    //         let node = Node::new(Some(vec));
    //         return get_iter(&Tree::Node(node));
    //       }

    //       let items = vec
    //         .iter()
    //         .map(|item| TreeIterator::new(&item))
    //         .fold(|item| item);

    //       items
    //     }
    //   }
    // }

    fn compare_optional_tree<T: PartialEq + Debug>(
      left: Option<&Tree<T>>,
      right: Option<&Tree<T>>,
    ) -> bool {
      match (left, right) {
        (Some(left), Some(right)) => left.compare_tree(right),
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => false,
      }
    }

    #[test]
    fn test_next() {
      // let leaf1 = leaf!("leaf1");
      // let leaf2 = leaf!("leaf2");

      // let mut node2 = node!("node2");

      // let mut node3 = node!("node3");
      // push_node(&mut node3, leaf!("leaf3"));

      // push_node(&mut node2, node3);

      // let mut node1 = node!("node1");
      // push_node(&mut node1, leaf1);
      // push_node(&mut node1, leaf2);
      // push_node(&mut node1, node2);

      // let mut root = node!("root");
      // push_node(&mut root, node1);

      let root = tree!([tree!(
        "node1",
        [
          tree!("leaf1"),
          tree!("leaf2"),
          tree!("node2", [tree!("node3", [tree!("leaf3")])])
        ]
      )]);

      let mut iter = TreeIterator::new(&root);

      let actual = iter.next();
      assert!(compare_optional_tree(actual, Some(&node!("node1"))));

      let actual = iter.next();
      println!("{:?}", actual);
      assert!(compare_optional_tree(actual, Some(&leaf!("leaf1"))));

      let actual = iter.next();
      assert_eq!(actual, None);
    }
  }
}
