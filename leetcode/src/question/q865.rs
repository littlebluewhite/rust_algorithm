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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    dfs(root).1
}
fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
        return (0, None);
    }
    let rc = node.unwrap();
    let (left, right) = {
        let tree_node = rc.borrow();
        (tree_node.left.clone(), tree_node.right.clone())
    };
    let (depth_left, left_node) = dfs(left);
    let (depth_right, right_node) = dfs(right);
    if depth_left == depth_right {
        (depth_right + 1, Some(rc))
    } else if depth_left > depth_right {
        (depth_left+1, left_node)
    } else {
        (depth_right+1, right_node)
    }
}
