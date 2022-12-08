/*
 * @lc app=leetcode.cn id=1662 lang=rust
 *
 * [1662] 检查两个字符串数组是否相等
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let s1 = word1.into_iter().collect::<String>();
        let s2 = word2.into_iter().collect::<String>();
        s1 == s2
    }
}
// @lc code=end
