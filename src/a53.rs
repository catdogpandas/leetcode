/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = -100000;
        nums.iter()
            .map(|&item| {
                pre = item.max(pre + item);
                pre
            })
            .max()
            .unwrap()
    }
}
// @lc code=end
