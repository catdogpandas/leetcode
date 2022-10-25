/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min = 100000;
        for item in prices.iter() {
            ans = ans.max(item - min);
            if min > *item {
                min = *item;
            }
        }
        ans
    }
}
// @lc code=end
