/*
 * @lc app=leetcode.cn id=剑指 Offer 09 lang=rust
 * @lcpr version=20603
 *
 * [剑指 Offer 09] 用两个栈实现队列
 */

// @lc code=start
struct CQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        Self {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.in_stack.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while !self.in_stack.is_empty() {
                self.out_stack.push(self.in_stack.pop().unwrap());
            }
            if self.out_stack.is_empty() {
                return -1;
            } else {
                return self.out_stack.pop().unwrap();
            }
        } else {
            return self.out_stack.pop().unwrap();
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
