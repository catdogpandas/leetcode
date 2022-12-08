/*
 * @lc app=leetcode.cn id=剑指 Offer 10- II lang=rust
 * @lcpr version=20801
 *
 * [剑指 Offer 10- II] 青蛙跳台阶问题
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        (0..n)
            .fold((0, 1), |res, x| (res.1, (res.0 + res.1) % 1000000007))
            .1
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 7\n
// @lcpr case=end

// @lcpr case=start
// 0\n
// @lcpr case=end

 */
