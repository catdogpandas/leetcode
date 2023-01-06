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
        if b.clone() == None {
            return false;
        }
        sub_structure(a.clone(), b.clone())
    }
}

pub fn sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let temp = equal_structure(a.clone(), b.clone());
    match a.clone() {
        Some(mut node) => {
            let left_tree = node.borrow_mut().left.take();
            let right_tree = node.borrow_mut().right.take();
            if temp || sub_structure(left_tree, b.clone()) || sub_structure(right_tree, b.clone()) {
                return true;
            } else {
                return false;
            }
        }
        None => {
            return false;
        }
    }
}

pub fn equal_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (Some(node1), Some(node2)) => {
            let main_val = node1.borrow().val;
            let branch_val = node2.borrow().val;
            if main_val == branch_val
                && equal_structure(node1.borrow().left.clone(), node2.borrow().left.clone())
                && equal_structure(node1.borrow().right.clone(), node2.borrow().right.clone())
            {
                //println!("{}",main_val);
                return true;
            } else {
                return false;
            }
        }
        (Some(node1), None) => {
            return true;
        }
        (None, Some(node2)) => {
            return false;
        }
        (None, None) => {
            return true;
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
