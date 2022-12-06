/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};
struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    history: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            cache: HashMap::new(),
            history: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.history.contains(&key) {
            self.history.retain(|&x| x != key);
        }
        if self.cache.contains_key(&key) {
            self.history.push_back(key);
        }
        *self.cache.get(&key).unwrap_or(&-1)
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.len() >= self.capacity && !self.cache.contains_key(&key) {
            self.cache.remove(&self.history[0]);
            let item = self.history[0];
            self.history.retain(|&x| x != item);
        }
        if self.history.contains(&key) {
            self.history.retain(|&x| x != key);
        }
        if !self.history.contains(&key) {
            self.history.push_back(key);
        }
        *self.cache.entry(key).or_insert(value) = value;
    }
}

// @lc code=end
