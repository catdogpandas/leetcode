/*
 * @lc app=leetcode.cn id=856 lang=rust
 *
 * [856] 括号的分数
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut depth = 0;
        let mut pre = false;
        for item in s.chars() {
            if item == '(' {
                depth += 1;
                pre = true;
            } else {
                depth -= 1;
                if pre {
                    ans += 1 << depth;
                }
                pre = false;
            }
        }

        ans
    }
}
// @lc code=end
