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

pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    let mut n = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        n += 1;
        cur = node.next.as_ref();
    }
    let half = n / 2;
    let mut cur = &mut head;
    for _ in 0..half {
        cur = &mut cur.as_mut().unwrap().next;
    }
    let second = cur.take();
    let mut reversed: Option<Box<ListNode>> = None;
    let mut cur = second;
    while let Some(mut node) = cur {
        let next = node.next.take();
        node.next = reversed;
        reversed = Some(node);
        cur = next;
    }
    let mut ans = 0;
    let mut left = head.as_ref();
    let mut right = reversed.as_ref();
    while let (Some(a), Some(b)) = (left, right){
        let sum = a.val + b.val;
        if sum > ans {
            ans = sum;
        }
        left = a.next.as_ref();
        right = b.next.as_ref();
    }
    ans
}