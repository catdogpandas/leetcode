/*
 * @lc app=leetcode.cn id=337 lang=rust
 *
 * [337] 打家劫舍 III
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node) = root {
                let val = node.as_ref().borrow().val;
                let left = &node.as_ref().borrow().left;
                let right = &node.as_ref().borrow().right;

                let (tl, fl) = dfs(left);
                let (tr, fr) = dfs(right);

                return (val + fl + fr, tl.max(fl) + tr.max(fr));
            }
            (0, 0)
        }
        let ans = dfs(&root);
        ans.0.max(ans.1)
    }
}
// @lc code=end
