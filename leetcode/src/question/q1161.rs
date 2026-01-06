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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none(){
        return 0
    }
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(root.unwrap());
    let mut best_sum = i64::MIN;
    let mut best_level = 1i32;
    let mut level = 1i32;
    while !q.is_empty(){
        let size = q.len();
        let mut sum = 0i64;
        for _ in 0..size{
            let node = q.pop_front().unwrap();
            let (val, left, right) = {
                let node = node.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };
            sum += val as i64;
            if let Some(left) = left{
                q.push_back(left);
            }
            if let Some(right) = right{
                q.push_back(right);
            }
        }
        if sum > best_sum{
            best_sum = sum;
            best_level = level;
        }
        level += 1;
    }
    best_level
}
