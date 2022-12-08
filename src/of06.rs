/*
 * @lc app=leetcode.cn id=剑指 Offer 06 lang=rust
 * @lcpr version=20603
 *
 * [剑指 Offer 06] 从尾到头打印链表
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
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut head = head;
        while let Some(node) = head {
            ans.push(node.val);
            head = node.next;
        }
        ans.reverse();
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,2]\n
// @lcpr case=end

 */
