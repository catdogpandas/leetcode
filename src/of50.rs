/*
 * @lc app=leetcode.cn id=剑指 Offer 50 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 50] 第一个只出现一次的字符
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        s.chars().for_each(|c| *cnt.entry(c).or_insert(0) += 1);
        for c in s.chars() {
            if cnt.get(&c) == Some(&1) {
                return c;
            }
        }
        ' '
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abaccdeff"\n
// @lcpr case=end

// @lcpr case=start
// ""\n
// @lcpr case=end

 */
