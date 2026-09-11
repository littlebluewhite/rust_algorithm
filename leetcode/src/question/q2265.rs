// #[derive(Debug, PartialEq, Eq)]
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

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if node.is_none() {
            return (0, 0, 0);
        }
        let node = node.unwrap();
        let val = node.borrow().val;
        let left = dfs(node.borrow().left.clone());
        let right = dfs(node.borrow().right.clone());
        let total = left.0 + right.0 + val;
        let count = left.1 + right.1 + 1;
        let mut res = left.2 + right.2;
        if total / count == val {
            res += 1;
        }
        (total, count, res)
    }
    dfs(root).2
}
