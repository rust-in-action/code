#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate libactionkv;
extern crate serde;

use std::collections::HashMap;
//use serde::{Serialize, Deserialize};
use libactionkv::ActionKV;

const ROOT_KEY: &[u8] = b"+root";
const NODE_KEY: &[u8] = b"+node";

#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;
type Value = Option<u64>; // we'll wrap values in options to allow us to sidestep the implementation of delete. delete(key) can become insert(key,
                          // None).

#[derive(Debug, Serialize, Deserialize)]
struct Node {
  keys: Vec<ByteString>,
  values: Vec<Value>,
  order: usize, /* also called _b_ for branching factor, _m_ for _m_-way tree (as a contraction from multi-way tree)
                 * and _n_ or _k_ for _n_-ary/_k_-ary tree from the word arity */
  is_root: bool,
  kind: NodeKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
enum NodeKind {
  Leaf,
  Branch, // Also called internal
}

impl Node {
  fn new(order: usize, kind: NodeKind, is_root: bool) -> Node {
    debug_assert!(order > 2);

    Node {
      keys: vec![],
      values: vec![],
      order: order,
      is_root: is_root,
      kind: kind,
    }
  }

  fn is_leaf(&self) -> bool {
    self.kind == NodeKind::Leaf
  }

  fn is_branch(&self) -> bool {
    self.kind == NodeKind::Branch
  }

  fn is_root(&self) -> bool {
    self.is_root
  }

  fn min_children(&self) -> usize {
    match (self.is_root(), self.is_leaf()) {
      (true, true) => 0,
      (true, false) => 2,
      (false, true) => (self.order / 2) - 1,
      (false, false) => (self.order / 2),
    }
  }

  fn max_children(&self) -> usize {
    match (self.is_root(), self.is_leaf()) {
      (true, true) => self.order - 1,
      (true, false) => self.order,
      (false, true) => self.order - 1,
      (false, false) => self.order,
    }
  }

  fn len(&self) -> usize {
    self.keys.len()
  }

  fn get(&self, key: &ByteString) -> Value {
    match self.keys.binary_search(key) {
      Ok(position) => self.values[position],
      Err(_) => None,
    }
  }

  fn insert(&mut self, key: &ByteString, value: u64) -> Option<Self> {
    self.insert_option_directly(key, Some(value))
  }

  fn insert_option_directly(&mut self, key: &ByteString, value: Value) -> Option<Self> {
    self.keys.sort_unstable(); // should be redundant, unstable here means that equivalent keys
    self.values.sort_unstable(); // could swap places. Unique [u8] will never compare as equivalent, so we can use this as a fast path that uses
                                 // constant memory.
    match self.keys.binary_search(key) {
      Ok(position) => self.values[position] = value, // found - update in-place
      Err(position) => {
        // the Err(position) provides the position where the item should be inserted to keep Vec<_> sorted
        self.keys.insert(position, key.clone());
        self.values.insert(position, value);
      }
    }

    let other = if self.keys.len() == self.max_children() {
      Some(self.split())
    } else {
      None
    };

    other
  }

  fn split(&mut self) -> Self {
    let mid = self.keys.len() / 2; // note: integer division

    let r_keys = self.keys.split_off(mid);
    let r_vals = self.values.split_off(mid);

    Node {
      keys: r_keys.to_vec(),
      values: r_vals.to_vec(),
      order: self.order,
      is_root: false,
      kind: self.kind,
    }
  }

  pub fn delete(&mut self, key: &ByteString) {
    self.insert_option_directly(key, None);
  }

  fn to_bincode(&self) -> ByteString {
    bincode::serialize(self).unwrap()
  }

  fn from_bincode(source: &ByteStr) -> Self {
    let mut future_self: Self = bincode::deserialize(source).unwrap();
    future_self.keys.sort_unstable(); // just in case something got lost in translation
    future_self.values.sort_unstable();
    future_self
  }
}

pub struct BPlusTree<'a> {
  order: usize,
  //root: Option<Node>,
  action_kv: &'a mut ActionKV, // lifetime 'a this means the ActionKV must live as long as BPlusTree
  root_position: Option<u64>,
}

#[derive(Serialize, Deserialize)] // because the
struct BPlusTreeParts {
  order: usize,
  root_position: Option<u64>,
}

impl<'a> BPlusTree<'a> {
  pub fn new(order: usize, root_position: Option<u64>, action_kv: &'a mut ActionKV) -> Self {
    assert!(order > 2);

    BPlusTree {
      order: order,
      action_kv: action_kv,
      root_position: root_position,
    }
  }

  pub fn insert(&mut self, key: &ByteString, value: u64) {
    self.insert_option_directly(key, Some(value))
  }

  pub fn delete(&mut self, key: &ByteString) {
    self.insert_option_directly(key, None)
  }

  fn insert_option_directly(&mut self, key: &ByteString, value: Value) {
    //const BACKEND_KEY: [u8; 6] = *b"+index";

    let (maybe_leaf, mut history) = self.find_leaf(key); // when matching a tuple, mut is provided before the variable name, rather than before the left parenthesis.

    let mut node_to_save = match (maybe_leaf, history.len()) {
      (Some(node), _) => node,
      (None, 0) => Node::new(self.order, NodeKind::Leaf, true),
      (None, _) => {
        debug_assert!(history[0].is_root());
        Node::new(self.order, NodeKind::Branch, false)
      }
    };

    while history.len() > 0 {
      let parent = history.pop();

      let maybe_new_node = node_to_save.insert_option_directly(key, value); // note: `Node` and `BPlusTree` both have insert()/insert_option_directly() methods
      if let Some(new_node_to_save) = maybe_new_node {
        let new_node_to_save_ = new_node_to_save.to_bincode();
        let new_node_position = self.action_kv.seek_to_end().unwrap();
        self.action_kv.insert(&NODE_KEY, &new_node_to_save_);

        //add_to_parent = Some((node_to_save.keys[0], new_node_position));

        if node_to_save.is_root() {
          debug_assert_eq!(history.len(), 0);

          node_to_save.is_root = false;
          let mut new_root = Node::new(self.order, NodeKind::Branch, true);
          new_root.insert(&new_node_to_save.keys[0], new_node_position);
          history.insert(0, new_root); // nodes will be added to this one via the normal process
        }
        //
        // parent.insert(new_leaf.keys[0], new_position); // will occassionally split
      }

      let node_to_save_ = node_to_save.to_bincode();
      let new_position = self.action_kv.seek_to_end().unwrap();
      self.action_kv.insert(&NODE_KEY, &node_to_save_); // okay for nodes in the tree to overlap their key in the underlying storage system, as they are not used within
                                                        // the btree itself

      let key = &node_to_save.keys[0];
      let value = Some(new_position);
      let node_to_save = parent;
    }
  }

  pub fn get(&mut self, key: &ByteString) -> Value {
    let (maybe_leaf, _history) = self.find_leaf(key);

    if let Some(leaf) = maybe_leaf {
      leaf.get(key) // return the result of looking in the leaf node (an Option, as the key may have been deleted even
                    // if it's present)
    } else {
      None // or return None if the key isn't in the tree
    }
  }

  fn find_leaf(&mut self, key: &ByteStr) -> (Option<Node>, Vec<Node>) {
    let mut nodes_visited: Vec<Node> = vec![];
    let next = match self.root_position {
      None => return (None, nodes_visited), // note: early return
      Some(position) => position,
    };

    loop {
      let kv = self.action_kv.get_at(next as u64).unwrap();
      let node_as_bytes = kv.value;
      let node = Node::from_bincode(&node_as_bytes);

      if node.is_leaf() {
        return (Some(node), nodes_visited);
      }

      if let Some(last_key_in_node) = node.keys.last() {
        // Vec<T>::last() returns Option is case the Vec<T> is empty
        if key >= last_key_in_node {
          let next = node.values.last().unwrap();
          continue;
        }
      }

      for (i, key_in_node) in node.keys.iter().enumerate() {
        if key < key_in_node {
          let next = node.values[i].unwrap(); // branch nodes can't be deleted
          break;
        }
      }
    }
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let fname = args.get(1).expect(&USAGE);
  let action = args.get(2).expect(&USAGE).as_ref();
  let key = args.get(3).expect(&USAGE).bytes().collect::<ByteString>();
  let maybe_value = args.get(4);

  let path = std::path::Path::new(&fname);
  let mut action_kv = ActionKV::open(path).expect("unable to open file");

  let mut index = match action_kv.find(ROOT_KEY) {
    Ok(Some((position, root))) => {
      let root = Node::from_bincode(&root);
      BPlusTree::new(root.order, Some(position), &mut action_kv)
    }
    Ok(None) => BPlusTree::new(80, None, &mut action_kv),
    Err(_) => panic!("unable to open file"),
  };

  match action {
    "get" => {
      match index.get(&key) {
        None => eprintln!("{:?} not found", key),
        Some(position) => {
          let kv = index.action_kv.get_at(position as u64).unwrap();
          println!("{:?}", kv.value);
        } // needs to use Debug as [u8] is arbitrary bytes
      }
    }
    // Other actions can actually remain as-is. In a long-standing application, it would be
    // necessary to clean up the index. As this utility is one-shot, it isn't essential here.
    "delete" => index.delete(&key),
    "insert" | "update" => {
      let value = maybe_value.expect(&USAGE).as_ref();
      let position = index.action_kv.insert_but_ignore_index(&key, value).unwrap();
      index.insert(&key, position);
    }
    _ => eprintln!("{}", &USAGE),
  }
}

mod test {
  use super::*;

  //    #[test]
  //    fn test_order_and_occupancy_rates() {
  //        let even = BPlusTree::new(8);
  //        assert_eq!(even.max_occupancy(), 7);
  //        assert_eq!(even.min_leaf_occupancy(), 4);
  //        assert_eq!(even.min_branch_occupancy(), 3);
  //
  //        let odd = BPlusTree::new(7);
  //        assert_eq!(odd.max_occupancy(), 6);
  //        assert_eq!(odd.min_leaf_occupancy(), 3);
  //        assert_eq!(odd.min_branch_occupancy(), 3);
  //    }
}
