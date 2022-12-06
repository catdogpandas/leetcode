/*
 * @lc app=leetcode.cn id=581 lang=rust
 *
 * [581] 最短无序连续子数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut cur = 0;
        cur = -1000000;
        for i in 0..nums.len() {
            if cur <= nums[i] {
                cur = nums[i]
            } else {
                r = i;
            }
        }
        cur = 1000000;
        for i in (0..nums.len()).rev() {
            if cur >= nums[i] {
                cur = nums[i];
            } else {
                l = i;
            }
        }
        if r == 0 {
            0
        } else {
            (r - l) as i32 + 1
        }
    }
}
// @lc code=end
