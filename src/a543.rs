/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] 二叉树的直径
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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        fn depth_of_tree(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(head) = root {
                let left_depth = depth_of_tree(head.deref().borrow().left.borrow(), ans);
                let right_depth = depth_of_tree(&head.deref().borrow().right, ans);
                *ans = (*ans).max(left_depth + right_depth);
                return left_depth.max(right_depth) + 1;
            }
            0
        }
        depth_of_tree(&root, &mut ans);
        ans
    }
}
// @lc code=end
