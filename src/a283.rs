/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        while slow < nums.len() {
            nums[slow] = 0;
            slow += 1;
        }
    }
}
// @lc code=end
