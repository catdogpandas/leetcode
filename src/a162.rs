/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut ans = 0;
        if nums.len() == 1 {
            return 0;
        }
        while left <= right {
            let mid = (left + right) / 2;
            if mid == 0 {
                if nums[mid] > nums[mid + 1] {
                    return 0;
                } else {
                    left = mid + 1;
                }
            } else if mid == nums.len() - 1 {
                if nums[mid] > nums[mid - 1] {
                    return mid as i32;
                } else {
                    right = mid - 1;
                }
            } else {
                if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
                    return mid as i32;
                } else if nums[mid] > nums[mid - 1] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
