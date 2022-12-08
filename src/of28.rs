/*
 * @lc app=leetcode.cn id=剑指 Offer 28 lang=rust
 * @lcpr version=20801
 *
 * [剑指 Offer 28] 对称的二叉树
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_symmetric(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if left.is_none() && right.is_none() {
                return true;
            }
            if let (Some(l), Some(r)) = (left, right) {
                if l.borrow().val == r.borrow().val {
                    return is_symmetric(&l.borrow().left, &r.borrow().right)
                        && is_symmetric(&l.borrow().right, &r.borrow().left);
                }
                return false;
            }
            false
        }
        is_symmetric(&root, &root)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,2,3,4,4,3]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,2,null,3,null,3]\n
// @lcpr case=end

 */
