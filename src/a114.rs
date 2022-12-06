/*
 * @lc app=leetcode.cn id=114 lang=rust
 *
 * [114] 二叉树展开为链表
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::flatten(&mut node.borrow_mut().left);
            Self::flatten(&mut node.borrow_mut().right);
            let right = node.borrow_mut().right.take();
            let left = node.borrow_mut().left.take();
            node.borrow_mut().right = left;
            let mut right_clone = node.clone();
            while right_clone.borrow().right.is_some() {
                let current = right_clone.borrow().right.clone().unwrap();
                right_clone = current;
            }
            right_clone.borrow_mut().right = right;
        }
    }
}
// @lc code=end
