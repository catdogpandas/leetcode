/*
 * @lc app=leetcode.cn id=剑指 Offer 26 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 26] 树的子结构
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
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => return false,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(a), Some(b)) => {
                if a.borrow().val == b.borrow().val {
                    if (b.borrow().left.is_some()
                        && Self::is_sub_structure(a.borrow().left.clone(), b.borrow().left.clone())
                        || b.borrow().left.is_none())
                        && (b.borrow().right.is_some()
                            && Self::is_sub_structure(
                                a.borrow().right.clone(),
                                b.borrow().right.clone(),
                            )
                            || b.borrow().right.is_none())
                    {
                        return true;
                    }
                }
                if Self::is_sub_structure(a.borrow().left.clone(), Some(b.clone())) {
                    return true;
                }
                if Self::is_sub_structure(a.borrow().right.clone(), Some(b)) {
                    return true;
                }

                return false;
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3]\n[3,1]\n
// @lcpr case=end

// @lcpr case=start
// [3,4,5,1,2]\n[4,1]\n
// @lcpr case=end

 */
