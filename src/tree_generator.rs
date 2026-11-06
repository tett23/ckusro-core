#![feature(conservative_impl_trait)]
#[macro_use]
use super::tree;
use super::tree::{Leaf, Node, Tree};
use std::fmt::Debug;
use std::ops::{Generator, GeneratorState};

// #[derive(Debug, PartialEq)]
// pub struct TreeVisitor<'a, T: PartialEq + Debug> {
//   tree: &'a Tree<T>,
//   current: &'a Tree<T>,
// }

// // #[derive(Debug, PartialEq)]
// impl<'a, T: PartialEq + Debug> TreeVisitor<'a, T> {
//   fn new(tree: &'a Tree<T>) -> TreeVisitor<T> {
//     TreeVisitor {
//       tree: tree,
//       current: tree,
//     }
//   }
// }

// // #[derive(Debug, PartialEq)]
// impl<'a, T: PartialEq + Debug> Iterator for TreeVisitor<'a, T> {
//   type Item = &'a Tree<T>;

//   fn next(&mut self) -> Option<&'a Tree<T>> {
//     let ret = match self.current {
//       Tree::Node(v) => {
//         v.children().iter()
//       },
//       Tree::Leaf(v) => None,
//     };

//     self.current = ret;

//     Some(&ret)
//   }
// }

// struct GenIterator<'a, G, T>(G);

// impl<'a, G, T: PartialEq + Debug> Iterator for GenIterator<'a, G, T>
// where
//   G: Generator<Yield = &'a Tree<T>, Return = ()>,
// {
//   type Item = G::Yield;

//   fn next(&mut self) -> Option<&'a Tree<T>> {
//     match unsafe { self.0.resume() } {
//       GeneratorState::Yielded(item) => Some(item),
//       GeneratorState::Complete(()) => None,
//     }
//   }
// }

struct GenIterator<G>(G);

impl<G> Iterator for GenIterator<G>
where
  G: Generator<Return = ()>,
{
  type Item = G::Yield;

  fn next(&mut self) -> Option<Self::Item> {
    match unsafe { self.0.resume() } {
      GeneratorState::Yielded(item) => Some(item),
      GeneratorState::Complete(()) => None,
    }
  }
}

fn tree_generator<'a, T: PartialEq + Debug>(root: &Tree<T>) -> impl Iterator<Item = &Tree<T>> {
  // let mut visitor = TreeVisitor::new(root);
  // println!("{:?}", visitor.next());

  let mut generator = || {
    match root {
      Tree::Node(node) => {
        yield node.at(0);
      }
      Tree::Leaf(l) => {
        yield &Tree::Leaf(*l);
      }
    }

    ()
  };

  // let ret = GenIterator::<generator.get,T>(generator);
  // let ret: impl Iterator<Item = &Tree<T>> = GenIterator(generator);

  GenIterator(generator)
}

#[cfg(test)]
mod tests {
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

  fn push_node<T: PartialEq + Debug>(node: &mut Tree<T>, value: Tree<T>) {
    match node {
      Tree::Leaf(_) => {}
      Tree::Node(node) => node.push(value),
    }
  }

  #[test]
  fn test_tree_generator() {
    let root = tree!([tree!(
      "node1",
      [
        tree!("leaf1"),
        tree!("leaf2"),
        tree!("node2", [tree!("node3", [tree!("leaf3")])])
      ]
    )]);

    let mut generator = tree_generator(&root);
    loop {
      match generator.next() {
        Some(v) => {
          let hoge = v;
          println!("{:?}", hoge);
        }
        None => break,
      };
    }
    // println!("{:?}", generator.next().unwrap());
    // unsafe { println!("{:?}", generator.next()) };
    // unsafe { println!("{:?}", generator.resume()) };
    // unsafe { println!("{:?}", generator.resume()) };
    // unsafe { println!("{:?}", generator.resume()) };
    // unsafe { println!("{:?}", generator.resume()) };
    // unsafe { println!("{:?}", generator.resume()) };
    // unsafe { println!("{:?}", generator.resume()) };
  }
}
