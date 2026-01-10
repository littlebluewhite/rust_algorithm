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
        let n = rc.borrow();
        (n.left.clone(), n.right.clone())
    };
    let (l_depth, l_node) = dfs(left);
    let (r_depth, r_node) = dfs(right);
    if l_depth == r_depth {
        (l_depth + 1, Some(rc))
    } else if r_depth > l_depth {
        (r_depth + 1, r_node)
    } else {
        (l_depth + 1, l_node)
    }
}
