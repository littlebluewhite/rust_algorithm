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

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_ref();
    let mut n = 0;
    while let Some(node) = cur {
        n += 1;
        cur = node.next.as_ref();
    }
    if n == 1 {
        return None;
    }
    let half = n / 2;
    let mut cur =&mut head;
    for _ in 0..half{
        cur = &mut cur.as_mut().unwrap().next
    }
    let mut removed = cur.take().unwrap();
    *cur = removed.next.take();
    head
}