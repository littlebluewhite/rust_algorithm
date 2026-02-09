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
pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut vals:Vec<i32> = Vec::new();
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>){
        if node.is_none(){
            return
        }
        let rc = node.unwrap();
        let n = rc.borrow();
        inorder(n.left.clone(), vals);
        vals.push(n.val);
        inorder(n.right.clone(), vals);

    }
    fn build(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>>{
        if vals.is_empty(){
            return None;
        }
        let mid = vals.len()/2;
        let mut root = TreeNode::new(vals[mid]);
        root.left = build(&vals[..mid]);
        root.right = build(&vals[mid+1..]);
        Some(Rc::new(RefCell::new(root)))
    }

    inorder(root, &mut vals);
    build(&vals)
}