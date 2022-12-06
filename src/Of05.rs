/*
 * @lc app=leetcode.cn id=剑指 Offer 05 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 05] 替换空格
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn replace_space(s: String) -> String {
        s.replace(" ", "%20")
    }
}
// @lc code=end

/*
// @lcpr case=start
// "We are happy."\n
// @lcpr case=end

 */
