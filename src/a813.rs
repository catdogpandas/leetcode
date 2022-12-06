/*
 * @lc app=leetcode.cn id=813 lang=rust
 *
 * [813] 最大平均值和的分组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut dp = vec![0_f64; nums.len() + 1];
        let mut pre = 0;
        let mut prefix: Vec<i32> = nums
            .iter()
            .map(|x| {
                pre = x + pre;
                pre
            })
            .collect();
        for i in 1..=nums.len() {
            dp[i] = prefix[i - 1] as f64 / i as f64;
        }
        for j in 2..=k as usize {
            for i in (j..=nums.len()).rev() {
                for x in j - 1..i {
                    dp[i] =
                        dp[i].max(dp[x] + (prefix[i - 1] - prefix[x - 1]) as f64 / (i - x) as f64);
                }
            }
        }
        dp[nums.len()]
    }
}
// @lc code=end
