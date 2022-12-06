/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(node) = root {
                let val = node.borrow().val;
                let left = &node.borrow().left;
                let right = &node.borrow().right;
                let left = dfs(left, ans);
                let right = dfs(right, ans);
                *ans = (*ans).max(val);
                if left > 0 && right > 0 {
                    let res = left + right + val;
                    *ans = (*ans).max(res);
                    return left.max(right) + val;
                } else if left > 0 {
                    *ans = (*ans).max(left + val);
                    return left + val;
                } else if right > 0 {
                    *ans = (*ans).max(right + val);
                    return right + val;
                } else {
                    return val;
                }
            }
            i32::MIN
        }
        let mut ans = i32::MIN;
        dfs(&root, &mut ans);
        ans
    }
}
// @lc code=end
