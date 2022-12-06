use std::borrow::BorrowMut;

/*
 * @lc app=leetcode.cn id=147 lang=rust
 *
 * [147] 对链表进行插入排序
 */
struct Solution;
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
// @lc code=start

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        while let Some(mut node) = head {
            head = node.next.take();
            let mut ptr = &mut dummy;
            while let Some(t) = &ptr.next {
                if t.val >= node.val {
                    break;
                }
                ptr = ptr.next.as_mut().unwrap();
            }
            node.next = ptr.next.take();
            ptr.next = Some(node);
        }
        dummy.next
    }
}
// @lc code=end
