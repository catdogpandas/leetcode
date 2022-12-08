/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy;
        let mut fast = &slow.clone();
        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        let remove = &mut slow.as_mut().unwrap().next;
        slow.as_mut().unwrap().next = remove.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}
// @lc code=end
