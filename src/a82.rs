use std::borrow::{Borrow, BorrowMut};

/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut p = res.as_mut().unwrap();
        let mut pre = 101;
        while let Some(mut node) = head {
            head = node.next.take();
            //如果当前访问的值与下一个值相等或与上一个值相等，则当前值不加进去。
            if (head.is_some() && head.as_ref().unwrap().val == node.val)
                || node.val == pre {
                pre = node.val;
            } else {
                pre = node.val;
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }
        res.as_mut().unwrap().next.take()
    }
}
// @lc code=end
