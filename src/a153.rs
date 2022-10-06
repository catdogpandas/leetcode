use core::num;

/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut ans = 5000;
        while left <= right {
            let mid = (left + right) / 2;
            ans = ans.min(nums[mid]);
            if nums[mid] < nums[0] {
                right = mid - 1;
            } else if nums[mid] > *nums.last().unwrap() {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return nums[mid];
                }
                right = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
