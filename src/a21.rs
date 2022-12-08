use std::borrow::Borrow;

/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */
struct Solution {}
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

// @lc code=start
// Definition for singly-linked list.

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut cur = &mut head;
        while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
            if n1.val < n2.val {
                cur.next = list1;
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2;
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }
        cur.next = if list1.is_some() { list1 } else { list2 };
        head.next
    }
}
// @lc code=end
