use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
  let mut map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
  let mut is_child: HashSet<i32> = HashSet::new();
  for description in descriptions {
    let parent = description[0];
    let child = description[1];
    let is_left = description[2] == 1;
    let parent_node = map.entry(parent).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent)))).clone();
    let child_node = map.entry(child).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child)))).clone();
    if is_left {
      parent_node.borrow_mut().left = Some(child_node.clone());
    } else {
      parent_node.borrow_mut().right = Some(child_node.clone());
    }

    is_child.insert(child);
  }
  map.iter().find(|(val, _)| !is_child.contains(val)).map(|(_, node)| node.clone())
}
