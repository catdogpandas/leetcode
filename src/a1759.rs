/*
 * @lc app=leetcode.cn id=1759 lang=rust
 * @lcpr version=21104
 *
 * [1759] 统计同构子字符串的数目
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        s.chars()
            .fold(('A', 0, 0), |mut pre, c| {
                if pre.0 == c {
                    pre.1 += 1;
                } else {
                    pre.1 = 1;
                }
                pre.0 = c;
                pre.2 += pre.1;
                pre.2 %= 1000000007;
                pre
            })
            .2
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abbcccaa"\n
// @lcpr case=end

// @lcpr case=start
// "xy"\n
// @lcpr case=end

// @lcpr case=start
// "zzzzz"\n
// @lcpr case=end

 */
