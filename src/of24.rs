/*
 * @lc app=leetcode.cn id=剑指 Offer 24 lang=rust
 * @lcpr version=20603
 *
 * [剑指 Offer 24] 反转链表
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
    pub fn reverse_list( head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut head = head;
        while let Some(mut node) = head {
            head=node.next.take();
            node.next=pre;
            pre = Some(node);
        }
        pre
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
