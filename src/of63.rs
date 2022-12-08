/*
 * @lc app=leetcode.cn id=剑指 Offer 63 lang=rust
 * @lcpr version=20801
 *
 * [剑指 Offer 63] 股票的最大利润
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |res, &x| {
                if x > res.0 {
                    (res.0, res.1.max(x - res.0))
                } else {
                    (x, res.1)
                }
            })
            .1
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

 */
