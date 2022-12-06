/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */
struct Solution;
// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        fn quick_select(nums: &mut Vec<i32>, l: usize, r: usize, k: usize) -> i32 {
            if l >= nums.len() || r >= nums.len() {
                return nums.len() as i32;
            }
            let mut p = l;
            for j in l..r {
                if nums[j] <= nums[r] {
                    nums.swap(p, j);
                    p += 1;
                }
            }
            nums.swap(p, r);
            if p == k {
                return nums[p];
            } else if p < k {
                return quick_select(nums, p + 1, r, k);
            } else {
                return quick_select(nums, l, p - 1, k);
            }
        }
        let mut nums = nums;
        let len = nums.len();
        quick_select(&mut nums, 0, len - 1, len - k as usize) as i32
    }
}
// @lc code=end
