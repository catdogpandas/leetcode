/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::{cell::RefCell, collections::HashMap, ptr::NonNull, rc::Rc};
struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: i32,
    size: i32,
    head: Option<Rc<RefCell<Node>>>,
    map: HashMap<i32, NonNull<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            size: 0,
            head: None,
            map: HashMap::new(),
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            
        }
        -1
    }

    fn put(&self, key: i32, value: i32) {}
}

// @lc code=end
