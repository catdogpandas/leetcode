/*
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * [876] 链表的中间结点
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
struct Solution { }
// @lc code=start
// Definition for singly-linked list.

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some()&&fast.as_ref().unwrap().next.is_some(){
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
// @lc code=end
