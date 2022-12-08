use std::{borrow::Borrow, ops::Deref};

/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */
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
struct Solution {}
// @lc code=start
// Definition for singly-linked list.

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
}
// @lc code=end
