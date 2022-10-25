/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        let mut count = 1;
        if let Some(node) = root {
            queue.push_back(node.clone());
        }
        while !queue.is_empty() {
            let mut cnt = count;
            count = 0;
            let mut tmp = vec![];
            while cnt > 0 {
                cnt -= 1;
                let node = queue.pop_front().unwrap();
                tmp.push(node.borrow_mut().val);
                if let Some(left) = node.borrow_mut().left.take() {
                    queue.push_back(left);
                    count += 1;
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    queue.push_back(right);
                    count += 1;
                };
            }
            ans.push(tmp);
        }
        ans
    }
}
// @lc code=end
