/*
 * @lc app=leetcode.cn id=155 lang=rust
 *
 * [155] 最小栈
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
    fn new() -> Self {
        Self {
            stack: vec![],
            cur_min: i64::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        let val = val as i64;
        if self.stack.is_empty() {
            self.stack.push(0);
            self.cur_min = val;
        } else {
            let tmp = val - self.cur_min;
            self.stack.push(tmp);
            if tmp < 0 {
                self.cur_min = val;
            }
        }
    }

    fn pop(&mut self) {
        let tmp = self.stack.pop().unwrap();
        if tmp < 0 {
            self.cur_min = self.cur_min - tmp;
        }
    }

    fn top(&self) -> i32 {
        let tmp = *self.stack.last().unwrap();
        if tmp < 0 {
            self.cur_min as i32
        } else {
            (tmp + self.cur_min) as i32
        }
    }

    fn get_min(&self) -> i32 {
        self.cur_min as i32
    }
}

// @lc code=end
