/*
 * @lc app=leetcode.cn id=448 lang=rust
 *
 * [448] 找到所有数组中消失的数字
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut tmp = vec![0; nums.len() + 1];
        for item in nums.iter() {
            tmp[*item as usize] = 1;
        }
        let n = nums.len() as i32;
        (1..=n)
            .filter(|idx| *idx != 0 && tmp[*idx as usize] == 0)
            .collect()
    }
}
// @lc code=end
