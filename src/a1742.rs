/*
 * @lc app=leetcode.cn id=1742 lang=rust
 *
 * [1742] 盒子中小球的最大数量
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut cnt = vec![0; 46];
        let mut max = -1;
        for i in low_limit..=high_limit {
            let (mut tmp, mut sum) = (i, 0);
            while tmp > 0 {
                sum += tmp % 10;
                tmp /= 10;
            }
            cnt[sum as usize] += 1;
            max = max.max(cnt[sum as usize])
        }
        max
    }
}
// @lc code=end
