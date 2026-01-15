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
    let total = sum(&root);
    let mut max = 0i128;
    dfs(&root, total, &mut max);
    (max % MOD) as i32
}

fn sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i64{
    if let Some(rc) = node{
        let (val, left, right) = {
            let tree_node = rc.borrow();
            (tree_node.val, tree_node.left.clone(), tree_node.right.clone())
        };
        val as i64 + sum(&left) + sum(&right)
    }else{
        0
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, total: i64, max: &mut i128) -> i64{
    if let Some(rc) = node{
        let (val, left, right) ={
            let tree_node = rc.borrow();
            (tree_node.val, tree_node.left.clone(), tree_node.right.clone())
        };
        let left = dfs(&left, total, max);
        let right = dfs(&right, total, max);
        let product = val as i64 + left + right;
        if product as i128 * (total-product) as i128 > *max{
            *max = product as i128;
        }
        product
    }else{
        0
    }
}