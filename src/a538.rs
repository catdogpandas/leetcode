/*
 * @lc app=leetcode.cn id=538 lang=rust
 *
 * [538] 把二叉搜索树转换为累加树
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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn bfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: i32) -> i32 {
            if let Some(head) = root {
                let cur = head.borrow().val + bfs(&head.borrow().right, prev);
                head.borrow_mut().val = cur;
                return bfs(&head.borrow().left, cur);
            }
            prev
        }
        bfs(&root, 0);
        root
    }
}
// @lc code=end
