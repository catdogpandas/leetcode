/*
 * @lc app=leetcode.cn id=1768 lang=rust
 *
 * [1768] 交替合并字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut l1 = word1.chars();
        let mut l2 = word2.chars();
        let mut ans = String::new();
        loop {
            match (l1.next(), l2.next()) {
                (Some(a), Some(b)) => {
                    ans.push(a);
                    ans.push(b);
                }
                (Some(ch), _) | (_, Some(ch)) => ans.push(ch),
                (None, None) => break,
            }
        }
        ans
    }
}
// @lc code=end
