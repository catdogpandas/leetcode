/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                if nums[mid] < nums[0] {
                    if mid == 0 {
                        return -1;
                    }
                    right = mid - 1;
                } else {
                    if target == nums[0] {
                        return 0;
                    } else if target < nums[0] {
                        left = mid + 1;
                    } else {
                        if mid == 0 {
                            return -1;
                        }
                        right = mid - 1;
                    }
                }
            } else {
                if nums[mid] >= nums[0] {
                    left = mid + 1;
                } else {
                    if target == nums[0] {
                        return 0;
                    } else if target > nums[0] {
                        if mid == 0 {
                            return -1;
                        }
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
            }
        }
        -1
    }
}
// @lc code=end
