/*
 * @lc app=leetcode.cn id=799 lang=rust
 *
 * [799] 香槟塔
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut poured = poured;
        let mut dp = vec![vec![0_f64; 100]; 100];
        dp[0][0] = poured as f64;
        for i in 0..query_row as usize {
            for j in 0..=i as usize {
                if dp[i][j] >= 1_f64 {
                    dp[i + 1][j] += (dp[i][j] - 1_f64) / 2_f64;
                    dp[i + 1][j + 1] += (dp[i][j] - 1_f64) / 2_f64;
                }
            }
        }
        dp[query_row as usize][query_glass as usize].min(1_f64)
    }
}
// @lc code=end
