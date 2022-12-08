/*
 * @lc app=leetcode.cn id=剑指 Offer 10- I lang=rust
 * @lcpr version=20801
 *
 * [剑指 Offer 10- I] 斐波那契数列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                (1..n)
                    .fold((0, 1), |a, x| {
                        (a.1 % 1000000007, (a.0 + a.1) % 1000000007)
                    })
                    .1
            }
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 5\n
// @lcpr case=end

 */
