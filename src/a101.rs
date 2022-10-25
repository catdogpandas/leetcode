/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
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
struct Solution {}
// @lc code=start

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(head) = root {
            let mut left_vec = vec![];
            let mut right_vec = vec![];
            left_vec.push(head.borrow_mut().left.take());
            right_vec.push(head.borrow_mut().right.take());
            loop {
                match (left_vec.pop(), right_vec.pop()) {
                    (Some(Some(left_node)), Some(Some(right_node))) => {
                        if left_node.borrow_mut().val == right_node.borrow_mut().val {
                            left_vec.push(left_node.borrow_mut().left.take());
                            left_vec.push(left_node.borrow_mut().right.take());
                            right_vec.push(right_node.borrow_mut().right.take());
                            right_vec.push(right_node.borrow_mut().left.take());
                        } else {
                            return false;
                        }
                    }
                    (Some(None), Some(None)) => continue,
                    (None, None) => break,
                    (_, _) => return false,
                }
            }
            return true;
        }
        true
    }
}
// @lc code=end
