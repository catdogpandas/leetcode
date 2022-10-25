/*
 * @lc app=leetcode.cn id=801 lang=rust
 *
 * [801] 使序列递增的最小交换次数
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut a = 0;
        let mut b = 1;
        for idx in 1..n {
            let at = a;
            let bt = b;
            a = n;
            b = n;
            if nums1[idx] > nums1[idx - 1] && nums2[idx] > nums2[idx - 1] {
                a = a.min(at);
                b = b.min(bt + 1);
            }
            if nums1[idx] > nums2[idx - 1] && nums2[idx] > nums1[idx - 1] {
                a = a.min(bt);
                b = b.min(at + 1);
            }
        }
        a.min(b) as i32
    }
}
// @lc code=end
