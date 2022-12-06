/*
 * @lc app=leetcode.cn id=1822 lang=rust
 *
 * [1822] 数组元素积的符号
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |res, &t| {
            if t > 0 {
                res
            } else if t < 0 {
                res * -1
            } else {
                0
            }
        })
    }
}
// @lc code=end
