/*
 * @lc app=leetcode.cn id=剑指 Offer 55 - I lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 55 - I] 二叉树的深度
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return Self::max_depth(node.borrow().left.clone())
                .max(Self::max_depth(node.borrow().right.clone()))
                + 1;
        }
        0
    }
}
// @lc code=end
