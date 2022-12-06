/*
 * @lc app=leetcode.cn id=891 lang=rust
 *
 * [891] 子序列宽度之和
 */
struct Solution;
// @lc code=start
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans: i64 = 0;
        let mut p2: i64 = 1;
        for i in 0..nums.len() {
            ans = (ans + ((nums[i] as i64) - (nums[nums.len() - i - 1] as i64)) * p2) % MOD;
            p2 = p2 * 2 % MOD;
        }
        ((ans + MOD) % MOD) as i32
    }
}
// @lc code=end
