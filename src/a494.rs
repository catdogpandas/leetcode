/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        fn dfs(nums: &Vec<i32>, idx: usize, cur: i32, target: i32) -> i32 {
            if idx == nums.len() {
                if cur == target {
                    return 1;
                } else {
                    return 0;
                }
            }
            dfs(nums, idx + 1, cur + nums[idx], target)
                + dfs(nums, idx + 1, cur - nums[idx], target)
        }
        dfs(&nums, 0, 0, target)
    }
}
// @lc code=end
