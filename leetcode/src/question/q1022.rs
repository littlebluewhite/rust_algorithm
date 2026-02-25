use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
    if let Some(r) = root{
        q.push_back((r, 0));
    }else{
        return 0
    }
    let mut ans = 0;
    while let Some((node, pre)) = q.pop_front(){
        let nc = node.borrow();
        let (val, left, right) = (nc.val, nc.left.clone(), nc.right.clone());
        let next = (pre << 1) | val;
        if left.is_none() && right.is_none() {
            ans += next;
            continue;
        }
        if let Some(l) = left{
            q.push_back((l, next));
        }
        if let Some(r) = right{
            q.push_back((r, next));
        }
    }
    ans
}

pub fn sum_root_to_leaf_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur: i32) -> i32{
        if let Some(rc) = node{
            let (val, left, right) = {
                let nc = rc.borrow();
                (nc.val, nc.left.clone(), nc.right.clone())
            };
            let next = (cur << 1) | val;
            if left.is_none() && right.is_none(){
                next
            }else{
                dfs(left, next) + dfs(right, next)
            }
        }else{
            0
        }
    }
    dfs(root, 0)
}