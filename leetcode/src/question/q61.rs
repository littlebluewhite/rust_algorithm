// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut len = 0usize;
    let mut cur = head.as_ref();
    while let Some(node) = cur{
        len += 1;
        cur = node.next.as_ref();
    }
    if len <= 1{
        return head;
    }
    let k = k as usize % len;
    if k == 0{
        return head;
    }
    let split = len -k;
    let mut cut = &mut head;
    for _ in 0..split-1{
        cut = &mut cut.as_mut().unwrap().next;
    }
    let mut new_head = cut.as_mut().unwrap().next.take();
    let mut tail = &mut new_head;
    while tail.as_ref().unwrap().next.is_some(){
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = head;
    new_head
}
