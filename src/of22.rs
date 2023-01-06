/*
 * @lc app=leetcode.cn id=剑指 Offer 22 lang=rust
 * @lcpr version=21001
 *
 * [剑指 Offer 22] 链表中倒数第k个节点
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

impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cnt = 0;
        let mut l = &head;
        let mut r = &head;
        while cnt < k {
            cnt += 1;
            l = &(l.as_ref().unwrap().next);
        }
        while l.is_some() {
            l = &(l.as_ref().unwrap().next);
            r = &(r.as_ref().unwrap().next);
        }
        (*r).clone()
    }
}
// @lc code=end
