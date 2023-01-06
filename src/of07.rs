/*
 * @lc app=leetcode.cn id=剑指 Offer 07 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 07] 重建二叉树
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn re_build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.len() == 0 {
                return None;
            }
            let head_val = preorder.first().unwrap();
            let mut iter = inorder.split(|num| num == head_val);
            let left_inorder = iter.next().unwrap();
            let right_inorder = iter.next().unwrap();
            let left_preorder = &preorder[1..1 + left_inorder.len()];
            let right_preorder = &preorder[(1 + left_inorder.len())..];
            let mut tmp_node = TreeNode::new(*head_val);
            tmp_node.left = re_build_tree(left_preorder, left_inorder);
            tmp_node.right = re_build_tree(right_preorder, right_inorder);
            return Some(Rc::new(RefCell::new(tmp_node)));
        }
        re_build_tree(&preorder.to_vec(), &inorder)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,9,20,15,7]\n[9,3,15,20,7]\n
// @lcpr case=end

// @lcpr case=start
// [-1]\n[-1]\n
// @lcpr case=end

 */
