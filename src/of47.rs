/*
 * @lc app=leetcode.cn id=剑指 Offer 47 lang=rust
 * @lcpr version=21001
 *
 * [剑指 Offer 47] 礼物的最大价值
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![-1; grid[0].len()];
        let mut l = -1;
        let mut ans = -1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if j == 0 {
                    l = 0;
                } else {
                    l = dp[j - 1];
                }
                if i == 0 {
                    dp[j] = grid[i][j] + l;
                    ans = ans.max(dp[j]);
                    continue;
                }
                dp[j] = l.max(dp[j]) + grid[i][j];
                ans = ans.max(dp[j]);
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
