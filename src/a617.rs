/*
 * @lc app=leetcode.cn id=617 lang=rust
 *
 * [617] 合并二叉树
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
struct Solution {}
// @lc code=start
// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut tmpnode2 = node2.as_ref().borrow_mut();
                let tmpnode1 = node1.as_ref().borrow();

                return Some(Rc::new(RefCell::new(TreeNode {
                    val: tmpnode1.val + tmpnode2.val,
                    left: Self::merge_trees(tmpnode1.left.clone(), tmpnode2.left.take()),
                    right: Self::merge_trees(tmpnode1.right.clone(), tmpnode2.right.take()),
                })));
            }
            (None, Some(node2)) => {
                return Some(node2);
            }
            (Some(node1), None) => {
                return Some(node1);
            }
            (None, None) => {
                return None;
            }
        }
    }
}
// @lc code=end
