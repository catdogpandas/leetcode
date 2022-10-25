/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let left = (nums1.len() + nums2.len() + 1) / 2;
        let right = (nums1.len() + nums2.len() + 2) / 2;
        return (Self::find_kth(&nums1, 0, &nums2, 0, left)
            + Self::find_kth(&nums1, 0, &nums2, 0, right)) as f64
            / 2.0;
    }
    fn find_kth(nums1: &Vec<i32>, i: usize, nums2: &Vec<i32>, j: usize, k: usize) -> i32 {
        if i >= nums1.len() {
            return nums2[j + k - 1];
        }
        if j >= nums2.len() {
            return nums1[i + k - 1];
        }
        if k == 1 {
            return nums1[i].min(nums2[j]);
        }
        let mid_val1 = if i + k / 2 - 1 < nums1.len() {
            nums1[i + k / 2 - 1]
        } else {
            i32::MAX
        };
        let mid_val2 = if j + k / 2 - 1 < nums2.len() {
            nums2[j + k / 2 - 1]
        } else {
            i32::MAX
        };
        if mid_val1 < mid_val2 {
            return Self::find_kth(nums1, i + k / 2, nums2, j, k - k / 2);
        } else {
            return Self::find_kth(nums1, i, nums2, j + k / 2, k - k / 2);
        }
    }
}
// @lc code=end
