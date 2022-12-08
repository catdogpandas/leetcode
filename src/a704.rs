use core::num;

/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */
pub struct Solution {}
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] == target {
                return mid as i32;
            } else {
                right = mid;
            }
        }
        if nums[left] == target {
            return left as i32;
        }
        -1
    }
}
// @lc code=end
