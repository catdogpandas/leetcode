/*
 * @lc app=leetcode.cn id=剑指 Offer 18 lang=rust
 * @lcpr version=21001
 *
 * [剑指 Offer 18] 删除链表的节点
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
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut head = &mut root;
        while let Some(node) = head {
            if node.val == val {
                *head = node.next.take();
                break;
            }
            head = &mut head.as_mut().unwrap().next;
        }
        root
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4,5,1,9]\n5\n
// @lcpr case=end

// @lcpr case=start
// [4,5,1,9]\n1\n
// @lcpr case=end

 */
