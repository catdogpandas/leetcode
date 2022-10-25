/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut t = (l1, l2, 0, 0);
        while t != (None, None, 0, 0) {
            t = match t {
                (Some(list1), Some(list2), _, carry) => (
                    list1.next,
                    list2.next,
                    (list1.val + list2.val + carry) % 10,
                    (list1.val + list2.val + carry) / 10,
                ),
                (Some(list1), None, _, carry) => (
                    list1.next,
                    None,
                    (list1.val + carry) % 10,
                    (list1.val + carry) / 10,
                ),
                (None, Some(list2), _, carry) => (
                    None,
                    list2.next,
                    (list2.val + carry) % 10,
                    (list2.val + carry) / 10,
                ),
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
            };
            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}
// @lc code=end
