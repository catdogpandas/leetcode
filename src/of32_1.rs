/*
 * @lc app=leetcode.cn id=剑指 Offer 32 - I lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 32 - I] 从上到下打印二叉树
 */
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let head = queue.pop_front().unwrap();
            if let Some(node) = head {
                ans.push(node.borrow().val);
                queue.push_back(node.as_ref().borrow().left.clone());
                queue.push_back(node.as_ref().borrow().right.clone());
            }
        }

        ans
    }
}
// @lc code=end
