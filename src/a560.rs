/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let pre = 0;
        *map.entry(pre).or_insert(0) += 1;
        nums.iter()
            .fold((0, 0), |(pre, res), &x| {
                let tmp = *map.entry(pre + x - k).or_insert(0);
                *map.entry(pre + x).or_insert(0) += 1;
                (pre + x, res + tmp)
            })
            .1
    }
}
// @lc code=end
