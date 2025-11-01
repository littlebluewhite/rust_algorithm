use std::collections::HashSet;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let ban: HashSet<i32> = nums.into_iter().collect();

    let mut dummy = Box::new(ListNode{val:0, next:head});

    let mut cur = &mut dummy;
    while let Some(mut node) = cur.next.take(){
        if ban.contains(&node.val) {
            cur.next = node.next.take();
        }else{
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy.next
}