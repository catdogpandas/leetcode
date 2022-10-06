/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */
struct Solution {}
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut ans = vec![];
        if nums.len() < 3 {
            return vec![];
        }
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                if nums[right] < 0 {
                    break;
                }
                if left > i + 1 && nums[left] == nums[left - 1] {
                    left += 1;
                    continue;
                }
                if right < nums.len() - 2 && nums[right] == nums[right + 1] {
                    right -= 1;
                    continue;
                }
                let tmp = nums[i] + nums[left] + nums[right];
                if tmp < 0 {
                    left += 1;
                } else if tmp > 0 {
                    right -= 1;
                } else {
                    ans.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
