/*
 * @lc app=leetcode.cn id=338 lang=rust
 *
 * [338] 比特位计数
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..=n {
            ans.push(i.count_ones() as i32);
        }
        ans
    }
}
// @lc code=end
