/*
 * @lc app=leetcode.cn id=940 lang=rust
 *
 * [940] 不同的子序列 II
 */
struct Solution;
// @lc code=start
const MOD: i32 = 1e9 as i32 + 7;
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        s.as_bytes().iter().fold(0, |sum, &ch| {
            let curr = (1 + sum - cnt[(ch - b'a') as usize] + MOD) % MOD;
            cnt[(ch - b'a') as usize] = (curr + cnt[(ch - b'a') as usize]) % MOD;
            (sum + curr) % MOD
        })
    }
}
// @lc code=end
