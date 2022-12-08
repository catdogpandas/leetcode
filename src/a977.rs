use core::num;

/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 双指针

        let mut ans = vec![0; nums.len()];
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut cur = nums.len() as i32 - 1;
        while cur >= 0 {
            if nums[left] * nums[left] >= nums[right] * nums[right] {
                ans[cur as usize] = nums[left] * nums[left];
                left += 1;
            } else {
                ans[cur as usize] = nums[right] * nums[right];
                right -= 1;
            }
            cur -= 1;
        }
        ans
    }
}
// @lc code=end
