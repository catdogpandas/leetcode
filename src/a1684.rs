/*
 * @lc app=leetcode.cn id=1684 lang=rust
 *
 * [1684] 统计一致字符串的数目
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        words.iter().fold(0, |sum, s| {
            for c in s.chars() {
                if allowed.find(c).is_none() {
                    return sum;
                }
            }
            sum + 1
        })
    }
}
// @lc code=end
