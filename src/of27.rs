/*
 * @lc app=leetcode.cn id=剑指 Offer 27 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 27] 二叉树的镜像
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
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(head) = root {
            let left = Self::mirror_tree(head.borrow_mut().left.take());
            let right = Self::mirror_tree(head.borrow_mut().right.take());
            head.borrow_mut().left = right;
            head.borrow_mut().right = left;
            return Some(head);
        }
        None
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
// [4,2,7,1,3,6,9]\n
// @lcpr case=end

 */
