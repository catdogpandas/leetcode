/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */
struct Solution {}
// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = nums.clone();
        for i in 0..nums.len() {
            if i > 1 {
                dp[i] = max(dp[i - 1], dp[i] + dp[i - 2]);
            }else if i==1{
                dp[i] = max(dp[i], dp[0]);
            }
        }
        *dp.last().unwrap()
    }
}
// @lc code=end
