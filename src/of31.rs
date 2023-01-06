/*
 * @lc app=leetcode.cn id=剑指 Offer 31 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 31] 栈的压入、弹出序列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut pos = 0;
        let mut ppos = 0;
        while pos < pushed.len() {
            stack.push(pushed[pos]);
            while !stack.is_empty() && *stack.last().unwrap() == popped[ppos] {
                stack.pop();
                ppos += 1;
            }
            pos += 1;
        }
        stack.is_empty()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4,5]\n[4,5,3,2,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,3,4,5]\n[4,3,5,1,2]\n
// @lcpr case=end

 */
