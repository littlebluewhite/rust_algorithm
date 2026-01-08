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
use std::rc::Rc;
const MOD: i128 = 1e9 as i128 + 7;
pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i64{
        if let Some(rc) = node{
            let (val, left, right) = {
                let n = rc.borrow();
                (n.val as i64, n.left.clone(), n.right.clone())
            };
            val + sum(&left) + sum(&right)
        }else{
            0
        }
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, total: i64, best: &mut i128) -> i64{
        if let Some(rc) = node{
            let (val, left, right) = {
                let n = rc.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            let left_sum = dfs(&left, total, best);
            let right_sum = dfs(&right, total, best);
            let sub_sum = left_sum + right_sum + val as i64;
            let product = sub_sum as i128 * ((total - sub_sum) as i128);
            if product > *best{
                *best = product;
            }
            sub_sum
        }else{
            0
        }
    }

    let total = sum(&root);
    let mut best = 0i128;
    dfs(&root, total, &mut best);
    (best % MOD) as i32
}