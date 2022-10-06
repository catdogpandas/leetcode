use std::vec;

/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans = vec![];
        if nums.len() < 4 {
            return vec![];
        }
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    if left > j + 1 && nums[left] == nums[left - 1] {
                        left += 1;
                        continue;
                    }
                    if right < nums.len() - 1 && nums[right] == nums[right + 1] {
                        right -= 1;
                        continue;
                    }
                    let tmp =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                    if tmp < target as i64 {
                        left += 1;
                    } else if tmp > target as i64 {
                        right -= 1;
                    } else {
                        ans.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
