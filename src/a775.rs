/*
 * @lc app=leetcode.cn id=775 lang=rust
 *
 * [775] 全局倒置与局部倒置
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut cur_max = -1;
        if nums.len()<=2{
            return true;
        }
        for i in 0..nums.len() - 2 {
            cur_max = cur_max.max(nums[i]);
            if cur_max > nums[i + 2] {
                return false;
            }
        }
        true
    }
}
// @lc code=end
