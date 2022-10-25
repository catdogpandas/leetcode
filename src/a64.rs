/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![i32::MAX; grid[0].len()];
        dp[0] = 0;
        for row in grid {
            for (idx, item) in row.iter().enumerate() {
                if idx == 0 {
                    dp[idx] += item;
                } else {
                    dp[idx] = dp[idx].min(dp[idx - 1]) + item;
                }
            }
        }
        *dp.last().unwrap()
    }
}
// @lc code=end
