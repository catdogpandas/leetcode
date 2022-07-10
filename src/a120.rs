/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] 三角形最小路径和
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let T = triangle.len();
        let mut dp = vec![0;T+1];

        for i in (0..T).rev() {
            for j in 0..=i {
                dp[j] = dp[j].min(dp[j+1]) + triangle[i][j];
            }
        }

        dp[0]
    }
}
// @lc code=end
