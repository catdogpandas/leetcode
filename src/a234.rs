use std::borrow::Borrow;

/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
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

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut valset = vec![];
        let mut head = head.as_ref();
        while let Some(node) = head {
            valset.push(node.val);
            head = node.next.as_ref();
        }
        for (cur, rev) in valset.iter().zip(valset.iter().rev()) {
            if cur != rev {
                return false;
            }
        }
        true
    }
}
// @lc code=end
