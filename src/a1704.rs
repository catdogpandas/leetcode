/*
 * @lc app=leetcode.cn id=1704 lang=rust
 *
 * [1704] 判断字符串的两半是否相似
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (l, r) = s.split_at(s.len() / 2);
        return if l.chars().into_iter().fold(0, |sum, c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                return sum + 1;
            }
            _ => sum,
        }) == r.chars().into_iter().fold(0, |sum, c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                return sum + 1;
            }
            _ => sum,
        }) {
            true
        } else {
            false
        };
    }
}
// @lc code=end
