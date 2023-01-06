/*
 * @lc app=leetcode.cn id=剑指 Offer 14- I lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 14- I] 剪绳子
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let exp = n as u32 / 3;
        match n as u32 % 3 {
            0 => 3_i32.pow(exp),
            1 => 4 * 3_i32.pow(exp - 1),
            2 => 2 * 3_i32.pow(exp),
            _ => -1,
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
