/*
 * @lc app=leetcode.cn id=剑指 Offer 34 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 34] 二叉树中和为某一值的路径
 */
struct Solution;
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
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut cur_vec = vec![];
        let mut ans = vec![];
        fn dfs(
            cur_vec: &mut Vec<i32>,
            root: Option<Rc<RefCell<TreeNode>>>,
            ans: &mut Vec<Vec<i32>>,
            cur_val: i32,
            target: i32,
        ) {
            if let Some(node) = root {
                cur_vec.push(node.borrow().val);
                if node.borrow().val + cur_val == target
                    && node.borrow().left.is_none()
                    && node.borrow().right.is_none()
                {
                    ans.push(cur_vec.clone());
                }
                dfs(
                    cur_vec,
                    node.borrow().left.clone(),
                    ans,
                    node.borrow().val + cur_val,
                    target,
                );
                dfs(
                    cur_vec,
                    node.borrow().right.clone(),
                    ans,
                    node.borrow().val + cur_val,
                    target,
                );
                cur_vec.pop();
            }
        }
        dfs(&mut cur_vec, root, &mut ans, 0, target);
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,4,8,11,null,13,4,7,2,null,null,5,1]\n22\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3]\n5\n
// @lcpr case=end

// @lcpr case=start
// [1,2]\n0\n
// @lcpr case=end

 */
