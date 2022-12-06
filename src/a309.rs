/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 最佳买卖股票时机含冷冻期
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut f0 = -prices[0];
        let mut f1 = 0;
        let mut f2 = 0;
        for i in 0..prices.len() {
            if i == 0 {
                f0 = -prices[0];
            } else if i == 1 {
                f2 = f0 + prices[1];
                f0 = (f1 - prices[i]).max(f0);
            } else {
                let tmp = f2;
                f2 = f0 + prices[i];
                f0 = (f1 - prices[i]).max(f0);
                f1 = f1.max(tmp);
            }
        }
        f1.max(f2)
    }
}
// @lc code=end
