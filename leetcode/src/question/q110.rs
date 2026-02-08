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

use std::rc::Rc;
use std::cell::RefCell;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  fn height_or_fail(node: Option<Rc<RefCell<TreeNode>>>) -> i32{
    if node.is_none(){
      return 0
    }

    let rc = node.unwrap();
    let (left, right) = {
      let n = rc.borrow();
      (n.left.clone(), n.right.clone())
    };
    let lh = height_or_fail(left);
    if lh == -1{
      return -1
    }
    let rh = height_or_fail(right);
    if rh == -1{
      return -1
    }
    if (lh-rh).abs() > 1{
      return -1
    }else{
      lh.max(rh) + 1
    }
  }
  height_or_fail(root) != -1
}