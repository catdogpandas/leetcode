/*
 * @lc app=leetcode.cn id=剑指 Offer 58 - II lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 58 - II] 左旋转字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let tmp = s.split_at(n as usize);
        tmp.1.to_string() + tmp.0
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abcdefg"\n2\n
// @lcpr case=end

// @lcpr case=start
// "lrloseumgh"\n6\n
// @lcpr case=end

 */
