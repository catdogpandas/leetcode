/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
 */
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
struct Solution;
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for item in lists {
            if let Some(head) = item {
                heap.push(Reverse(head));
            }
        }
        let mut ans = Box::new(ListNode::new(0));
        let mut cur = &mut ans;
        while let Some(Reverse(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(Reverse(next));
            }
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }

        ans.next
    }
}
// @lc code=end
