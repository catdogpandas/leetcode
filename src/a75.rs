/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i < nums.len() {
            match nums[i] {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                }
                1 => {
                    i += 1;
                }
                2 if i < right => {
                    nums.swap(i, right);
                    right -= 1;
                }
                _ => break,
            }
        }
    }
}
// @lc code=end
