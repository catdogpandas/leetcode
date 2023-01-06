/*
 * @lc app=leetcode.cn id=剑指 Offer 54 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 54] 二叉搜索树的第k大节点
 */
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
// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn find_kth(root: &Option<Rc<RefCell<TreeNode>>>, cur: &mut i32, k: i32) -> Option<i32> {
            if let Some(node) = root {
                let right = find_kth(&node.borrow().right, cur, k);
                if right.is_some() {
                    return right;
                }
                if *cur + 1 == k {
                    return Some(node.borrow().val.clone());
                }
                *cur = *cur + 1;
                let left = find_kth(&node.borrow().left, cur, k);
                if left.is_some() {
                    return left;
                }
                return None;
            }
            None
        }
        let mut cur = 0;
        find_kth(&root, &mut cur, k).unwrap()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,1,4,null,2]\n13/ \1   4\2\n
// @lcpr case=end

// @lcpr case=start
// [5,3,6,2,4,null,null,1]\n35/ \3   6/ \2   4/1\n
// @lcpr case=end

 */
