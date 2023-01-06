/*
 * @lc app=leetcode.cn id=剑指 Offer 62 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 62] 圆圈中最后剩下的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut f = 0;
        for i in 2..(n + 1) {
            f = (f + m) % i;
        }
        f
    }
}
// @lc code=end

/*
// @lcpr case=start
// 5\n3\n
// @lcpr case=end

// @lcpr case=start
// 10\n17\n
// @lcpr case=end

 */
