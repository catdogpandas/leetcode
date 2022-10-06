/*
 * @lc app=leetcode.cn id=34 lang=rust&
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn bianry_left_search(nums: &Vec<i32>, target: i32) -> i32 {
            let mut ans = -1;
            if nums.len() == 0 {
                return -1;
            }
            let mut left = 0;
            let mut right = nums.len() as i32 - 1;
            while left <= right {
                let mid = (left + right) / 2;
                if nums[mid as usize] >= target {
                    if nums[mid as usize] == target {
                        ans = mid;
                    }
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            ans
        }
        fn bianry_right_search(nums: &Vec<i32>, target: i32) -> i32 {
            let mut ans = -1;
            if nums.len() == 0 {
                return -1;
            }
            let mut left = 0;
            let mut right = nums.len() as i32 - 1;
            while left <= right {
                let mid = (left + right) / 2;
                if nums[mid as usize] > target {
                    right = mid - 1;
                } else {
                    if nums[mid as usize] == target {
                        ans = mid;
                    }
                    left = mid + 1;
                }
            }
            ans
        }
        return vec![
            bianry_left_search(&nums, target),
            bianry_right_search(&nums, target),
        ];
    }
}
// @lc code=end
