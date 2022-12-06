/*
 * @lc app=leetcode.cn id=297 lang=rust
 *
 * [297] 二叉树的序列化与反序列化
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
// @lc code=start

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();
        fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
            if let Some(root) = root {
                ans.push_str(&root.borrow().val.to_string());
                ans.push(',');
                preorder_traversal(root.borrow_mut().left.take(), ans);
                preorder_traversal(root.borrow_mut().right.take(), ans);
            } else {
                ans.push_str("X,")
            }
        }
        preorder_traversal(root, &mut ans);
        ans.pop();
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut data: Vec<_> = data.split(',').rev().collect();
        fn dfs(data: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
            let cur = data.pop().unwrap();
            if cur == "X" {
                return None;
            }
            let root = Rc::new(RefCell::new(TreeNode::new(cur.parse::<i32>().unwrap())));
            root.borrow_mut().left = dfs(data);
            root.borrow_mut().right = dfs(data);
            Some(root)
        }
        dfs(&mut data)
    }
}
// @lc code=end
