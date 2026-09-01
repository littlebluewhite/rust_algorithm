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

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut prev = match head.as_ref() {
        Some(node) => node,
        None => return vec![-1, -1],
    };
    let mut curr = match prev.next.as_ref() {
        Some(node) => node,
        None => return vec![-1, -1],
    };
    let mut index = 2i32;
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;
    let mut min_dist = i32::MAX;
    while let Some(next) = curr.next.as_ref() {
        let critical = (prev.val > curr.val && next.val > curr.val) || (prev.val < curr.val && next.val < curr.val);
        if critical {
            if let Some(&prev_critical) = last.as_ref() {
                min_dist = min_dist.min(index - prev_critical);
            }
            if first.is_none() {
                first = Some(index);
            }
            last = Some(index);
        }
        prev = curr;
        curr = next;
        index += 1;
    }
    match (first, last) {
        (Some(start), Some(end)) if start != end =>{
            return vec![min_dist, end - start];
        },
        _ => return vec![-1, -1],
    }
}