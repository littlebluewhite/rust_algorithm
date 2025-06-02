use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
#[derive(Eq, PartialEq)]
struct HeapNode(Box<ListNode>);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 注意：BinaryHeap 是 max-heap，這裡要反過來「最小值優先」
        other.0.val.cmp(&self.0.val)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(
    lists: Vec<Option<Box<ListNode>>>
) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();

    // Add the head of each linked list to the heap
    for list in lists.into_iter() {
        if let Some(node) = list {
            heap.push(HeapNode(node));
        }
    }
    // Create a dummy node to simplify operations
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // Process nodes in the heap
    while let Some(HeapNode(mut node)) = heap.pop() {
        // If there is a next node, push it to the heap
        if let Some(next) = node.next.take() {
            heap.push(HeapNode(next));
        }
        // Append the current node to the result list
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }

    // Return the merged list, skipping the dummy node
    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 將 Vec<i32> 轉成鍊表
    fn from_vec(v: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for &x in v {
            tail.next = Some(Box::new(ListNode::new(x)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    /// 將鍊表轉成 Vec<i32>
    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut cur = list;
        while let Some(node) = cur {
            res.push(node.val);
            cur = node.next;
        }
        res
    }

    #[test]
    fn test_empty_lists() {
        // 測試當輸入的 lists 為空（k = 0）時，應該回傳 None
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let merged = merge_k_lists(lists);
        assert!(merged.is_none());
    }
    #[test]
    fn test_all_empty() {
        // 測試所有鏈表皆為空的情況
        let lists = vec![None, None, None];
        let merged = merge_k_lists(lists);
        assert!(merged.is_none());
    }

    #[test]
    fn test_single_list() {
        // 測試只有一條鏈表的情況，應該直接回傳該鏈表
        let lists = vec![from_vec(&[1, 2, 3])];
        let merged = merge_k_lists(lists);
        assert_eq!(to_vec(merged), vec![1, 2, 3]);
    }

    #[test]
    fn test_multiple_lists() {
        // Example: [[1,4,5], [1,3,4], [2,6]]
        let lists = vec![
            from_vec(&[1, 4, 5]),
            from_vec(&[1, 3, 4]),
            from_vec(&[2, 6]),
        ];
        let merged = merge_k_lists(lists);
        assert_eq!(to_vec(merged), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_varied_lengths() {
        // 不同長度、含重複值
        let lists = vec![
            from_vec(&[5, 10]),
            None,
            from_vec(&[2, 2, 2]),
            from_vec(&[3]),
        ];
        let merged = merge_k_lists(lists);
        assert_eq!(to_vec(merged), vec![2, 2, 2, 3, 5, 10]);
    }
}