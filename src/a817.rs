/*
 * @lc app=leetcode.cn id=817 lang=rust
 *
 * [817] 链表组件
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
struct Solution {}
// @lc code=start
use std::{borrow::Borrow, collections::HashSet};
impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut num_set = HashSet::new();
        for item in nums {
            num_set.insert(item);
        }
        let mut ans = 0;
        let mut length = 0;
        let mut head = &head;
        while let Some(node) = head {
            let val = node.val;
            if num_set.contains(&val) {
                length += 1;
            } else {
                if length != 0 {
                    ans += 1;
                    length = 0;
                }
            }
            head = node.next.borrow();
        }
        if length > 0 {
            ans += 1;
        }
        ans
    }
}
// @lc code=end
