/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 {
            if nums[i - 1] < nums[i] {
                let mut j = i;
                while j < nums.len() && nums[i - 1] < nums[j] {
                    j += 1;
                }
                nums.swap(i - 1, j - 1);
                break;
            }
            i -= 1;
        }
        nums[i..].sort();
    }
}
// @lc code=end
