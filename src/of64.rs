/*
 * @lc app=leetcode.cn id=剑指 Offer 64 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 64] 求1+2+…+n
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        let mut n = n;
        n > 0 && {
            n += Solution::sum_nums(n - 1);
            true
        };
        n
    }
}
// @lc code=end

/*
// @lcpr case=start
// 3\n
// @lcpr case=end

// @lcpr case=start
// 9\n
// @lcpr case=end

 */
