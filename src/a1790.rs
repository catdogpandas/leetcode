/*
 * @lc app=leetcode.cn id=1790 lang=rust
 *
 * [1790] 仅执行一次字符串交换能否使两个字符串相等
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut v1 = vec![];
        let mut v2 = vec![];
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                v1.push(c1);
                v2.push(c2);
            }
            if v1.len() > 2 {
                return false;
            }
        }
        if v1.is_empty() {
            return true;
        }
        if v1.len() == 2 {
            if v1[0] == v2[1] && v1[1] == v2[0] {
                return true;
            }
        }
        false
    }
}
// @lc code=end
