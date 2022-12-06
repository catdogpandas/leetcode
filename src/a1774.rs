/*
 * @lc app=leetcode.cn id=1774 lang=rust
 *
 * [1774] 最接近目标价格的甜点成本
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut topping_costs = topping_costs;
        topping_costs.append(&mut topping_costs.clone());
        topping_costs.sort();
        let base_min = *base_costs.iter().min().unwrap();
        if base_min >= target {
            return base_min;
        }
        let mut res = 2 * target - base_min;
        let mut dp = vec![false; target as usize + 1];
        for item in base_costs {
            if item <= target {
                dp[item as usize] = true;
            } else {
                res = res.min(item);
            }
        }
        for item in topping_costs {
            for idx in (0..=target).rev() {
                if dp[idx as usize] && idx + item > target {
                    res = res.min(idx + item);
                }
                if idx - item > 0 {
                    dp[idx as usize] |= dp[(idx - item) as usize]
                }
            }
        }
        for i in 0..=res - target {
            if dp[(target - i) as usize] {
                return target - i;
            }
        }
        res
    }
}
// @lc code=end
