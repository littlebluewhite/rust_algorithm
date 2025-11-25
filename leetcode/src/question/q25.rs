// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

pub fn reverse_k_group(
    head: Option<Box<ListNode>>,
    k: i32
) -> Option<Box<ListNode>> {
    let k = k as usize;
    // 如果 k < 2，不用翻轉，直接回傳
    if k < 2 {
        return head;
    }
    // dummy.next 指向真實的 head
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    // 這裡用 &mut borrow dummy，而不是把它搬走
    let mut group_prev: &mut Box<ListNode> = &mut dummy;

    loop {
        // 找第 k 個節點 (kth)──只借用，不動它的所有權
        let mut kth = &mut *group_prev;
        for _ in 0..k {
            // as_mut() 回傳 Option<&mut Box<ListNode>>
            if let Some(next_node) = kth.next.as_mut() {
                kth = next_node;
            } else {
                // 不足 k 個，結束
                return dummy.next;
            }
        }

        // 把 kth.next.take() 移走的是「下一組的頭」，不是整個 group_prev
        let group_next = kth.next.take();

        // 反轉那 k 個節點──curr, prev 都是 local 變數
        let mut prev = group_next;
        let mut curr = group_prev.next.take();
        for _ in 0..k {
            if let Some(mut node) = curr {
                curr = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
        }

        // 接回翻轉後的段落
        group_prev.next = prev;

        // 把 group_prev 往前移 k 步──再借用，不移動所有權
        for _ in 0..k {
            group_prev = group_prev.next.as_mut().unwrap();
        }
        // 接下來繼續處理下一段
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 將 Vec 轉成鍊表
    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        // 反向插入
        for &val in v.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// 將鍊表轉回 Vec
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        v
    }

    #[test]
    fn test_example1_k2() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let res  = reverse_k_group(head, 2);
        assert_eq!(list_to_vec(res), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_example2_k3() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let res  = reverse_k_group(head, 3);
        assert_eq!(list_to_vec(res), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn test_empty_list() {
        let head: Option<Box<ListNode>> = None;
        let res = reverse_k_group(head, 3);
        assert_eq!(list_to_vec(res), Vec::<i32>::new());
    }

    #[test]
    fn test_k_equals_one() {
        let head = vec_to_list(vec![1, 2, 3]);
        let res  = reverse_k_group(head.clone(), 1);
        // k = 1 應該回傳完全相同的串列
        assert_eq!(list_to_vec(res), list_to_vec(head));
    }

    #[test]
    fn test_k_greater_than_length() {
        let head = vec_to_list(vec![1, 2, 3]);
        let res  = reverse_k_group(head.clone(), 5);
        // 長度不足 k 時，不做任何翻轉
        assert_eq!(list_to_vec(res), list_to_vec(head));
    }
}