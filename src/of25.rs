/*
 * @lc app=leetcode.cn id=剑指 Offer 25 lang=rust
 * @lcpr version=21006
 *
 * [剑指 Offer 25] 合并两个排序的链表
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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = ListNode::new(0);
        let mut ptr = &mut head.next;
        loop {
            match (l1, l2) {
                (Some(mut a1), Some(mut a2)) => {
                    if a1.val < a2.val {
                        l1 = a1.next.take();
                        *ptr = Some(a1);
                        l2 = Some(a2);
                    } else {
                        l2 = a2.next.take();
                        *ptr = Some(a2);
                        l1 = Some(a1);
                    }
                    ptr = &mut ptr.as_mut().unwrap().next;
                }
                (None, None) => break,
                (None, Some(n)) | (Some(n), None) => {
                    *ptr = Some(n);
                    break;
                }
            }
        }
        head.next
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
