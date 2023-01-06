/*
 * @lc app=leetcode.cn id=面试题59 - II lang=rust
 * @lcpr version=21105
 *
 * [面试题59 - II] 队列的最大值
 */

// @lc code=start
use std::collections::VecDeque;
struct MaxQueue {
    cur_max_queue: VecDeque<i32>,
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    fn new() -> Self {
        Self {
            cur_max_queue: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }

    fn max_value(&self) -> i32 {
        if self.queue.is_empty() {
            -1
        } else {
            *self.cur_max_queue.front().unwrap()
        }
    }

    fn push_back(&mut self, value: i32) {
        self.queue.push_back(value);
        while !self.cur_max_queue.is_empty() && *self.cur_max_queue.back().unwrap() < value {
            self.cur_max_queue.pop_back();
        }
        self.cur_max_queue.push_back(value);
    }

    fn pop_front(&mut self) -> i32 {
        if self.queue.is_empty() {
            return -1;
        }
        let res = self.queue.pop_front().unwrap();
        if res == *self.cur_max_queue.front().unwrap() {
            self.cur_max_queue.pop_front();
        }
        res
    }
}

// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

 */
