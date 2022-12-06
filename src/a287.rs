/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
            fast = nums[fast] as usize;
            if slow == fast {
                break;
            }
        }
        slow = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        slow as i32
    }
}
// @lc code=end
