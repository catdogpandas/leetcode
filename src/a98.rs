/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
            if let Some(head) = root {
                let val = head.borrow().val as i64;
                if val > left && val < right {
                    return dfs(&head.borrow().left, left, val)
                        && dfs(&head.borrow().right, val, right);
                }
                return false;
            }
            true
        }
        dfs(&root, i64::MIN, i64::MAX)
    }
}
// @lc code=end
