/*
 * @lc app=leetcode.cn id=剑指 Offer 30 lang=rust
 * @lcpr version=20603
 *
 * [剑指 Offer 30] 包含min函数的栈
 */

// @lc code=start
struct MinStack {
    stack: Vec<i64>,
    cur_min: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            cur_min: i64::MAX,
        }
    }

    fn push(&mut self, x: i32) {
        let x = x as i64;
        if self.stack.is_empty() {
            self.stack.push(0);
            self.cur_min = x;
        } else {
            let tmp = x - self.cur_min;
            if tmp < 0 {
                self.cur_min = x;
            }
            self.stack.push(tmp);
        }
    }

    fn pop(&mut self) {
        if !self.stack.is_empty() {
            let last = self.stack.pop().unwrap();
            if last < 0 {
                self.cur_min = self.cur_min - last;
            }
        }
    }

    fn top(&self) -> i32 {
        if !self.stack.is_empty() {
            let last = *self.stack.last().unwrap();
            if last < 0 {
                return self.cur_min as i32;
            } else {
                return (last + self.cur_min) as i32;
            }
        }
        -1
    }

    fn min(&self) -> i32 {
        self.cur_min as i32
    }
}

// @lc code=end
