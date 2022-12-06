/*
 * @lc app=leetcode.cn id=221 lang=rust
 *
 * [221] 最大正方形
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut ans: i32 = 0;
        let mut dp = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| match c {
                        '0' => 0,
                        _ => 1,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                if i == 0 || j == 0 {
                    ans = ans.max(dp[i][j]);
                    continue;
                }
                let a = dp[i][j - 1];
                let b = dp[i - 1][j];
                let c = dp[i - 1][j - 1];
                let min = a.min(b).min(c);
                if min > 0 && dp[i][j] > 0 {
                    dp[i][j] = min + 1;
                }
                ans = ans.max(dp[i][j]);
            }
        }
        ans.pow(2)
    }
}
// @lc code=end
