/*
 * @lc app=leetcode.cn id=1805 lang=rust
 * @lcpr version=20702
 *
 * [1805] 字符串中不同整数的数目
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('0'))
            .collect::<std::collections::HashSet<_>>()
            .len() as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// "a123bc34d8ef34"\n
// @lcpr case=end

// @lcpr case=start
// "leet1234code234"\n
// @lcpr case=end

// @lcpr case=start
// "a1b01c001"\n
// @lcpr case=end

 */
