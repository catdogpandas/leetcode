/*
 * @lc app=leetcode.cn id=剑指 Offer 58 - I lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 58 - I] 翻转单词顺序
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

 */
