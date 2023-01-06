/*
 * @lc app=leetcode.cn id=剑指 Offer 41 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 41] 数据流中的中位数
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct MedianFinder {
    heap_max: BinaryHeap<Reverse<i32>>,
    heap_min: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            heap_max: BinaryHeap::new(),
            heap_min: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.heap_min.is_empty() || *self.heap_min.peek().unwrap() >= num {
            self.heap_min.push(num);
            if self.heap_min.len() > self.heap_max.len() + 1 {
                let val = self.heap_min.pop().unwrap();
                self.heap_max.push(Reverse(val));
            }
        } else {
            self.heap_max.push(Reverse(num));
            if self.heap_max.len() > self.heap_min.len() {
                let val = self.heap_max.pop().unwrap().0;
                self.heap_min.push(val);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.heap_min.len() > self.heap_max.len() {
            *self.heap_min.peek().unwrap() as f64
        } else {
            (*self.heap_min.peek().unwrap() + self.heap_max.peek().unwrap().0) as f64 / 2.0
        }
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
