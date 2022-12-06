/*
 * @lc app=leetcode.cn id=312 lang=rust
 *
 * [312] 戳气球
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut coins = vec![1; n + 2];
        coins[1..=n].clone_from_slice(&nums);
        let mut dp = vec![vec![0; n + 2]; n + 2];

        for i in (0..=n).rev() {
            for j in i + 1..=n + 1 {
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + coins[i] * coins[k] * coins[j] + dp[k][j]);
                }
            }
        }

        dp[0][n + 1]
    }
}
// @lc code=end
