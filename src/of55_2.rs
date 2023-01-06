/*
 * @lc app=leetcode.cn id=剑指 Offer 55 - II lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 55 - II] 平衡二叉树
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_b(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            if let Some(node) = root {
                let left = is_b(&node.borrow().left);
                let right = is_b(&node.borrow().right);
                if left.0 && right.0 && (left.1 - right.1).abs() < 2 {
                    return (true, left.1.max(right.1) + 1);
                }
                return (false, left.1.max(right.1) + 1);
            }
            (true, 0)
        }
        is_b(&root).0
    }
}
// @lc code=end
