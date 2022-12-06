/*
 * @lc app=leetcode.cn id=895 lang=rust
 *
 * [895] 最大频率栈
 */

// @lc code=start
use std::collections::HashMap;
struct FreqStack {
    count: HashMap<i32, i32>,
    stack: HashMap<i32, Vec<i32>>,
    cur_max: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            count: HashMap::new(),
            stack: HashMap::new(),
            cur_max: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let p = self.count.entry(val).or_insert(0);
        *p += 1;
        self.cur_max = self.cur_max.max(*p);
        let p_stack = self.stack.entry(*p).or_insert(vec![]);
        p_stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        let p_stack = self.stack.entry(self.cur_max).or_default();
        let res = p_stack.pop().unwrap();
        let p = self.count.entry(res).or_default();
        *p -= 1;
        if p_stack.is_empty() {
            self.cur_max -= 1;
        }
        res
    }
}

// @lc code=end
