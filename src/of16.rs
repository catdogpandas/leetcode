/*
 * @lc app=leetcode.cn id=剑指 Offer 16 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 16] 数值的整数次方
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut pre = x;
        let mut ans = 1.0;
        let mut p = (n as i64).abs();
        if n == 0 {
            return 1.0;
        }
        if n > 0 {
            pre = x;
        } else {
            pre = 1.0 / x;
        }
        while p > 0 {
            if p & 1 == 1 {
                ans = ans * pre;
            }
            p = p >> 1;
            pre = pre * pre;
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2.00000\n10\n
// @lcpr case=end

// @lcpr case=start
// 2.10000\n3\n
// @lcpr case=end

// @lcpr case=start
// 2.00000\n-2\n
// @lcpr case=end

 */
