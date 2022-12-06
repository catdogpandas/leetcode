/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut cur = 1;
        let mut suffix_mul = nums
            .iter()
            .rev()
            .map(|&x| {
                cur = cur * x;
                cur
            })
            .collect::<Vec<_>>();
        suffix_mul.reverse();
        cur = nums[0];
        for i in 0..suffix_mul.len() {
            if i == 0 {
                cur = nums[0];
                suffix_mul[0] = suffix_mul[1];
            } else if i == suffix_mul.len() - 1 {
                suffix_mul[i] = cur;
            } else {
                suffix_mul[i] = cur * suffix_mul[i + 1];
                cur = cur * nums[i];
            }
        }
        suffix_mul
    }
}
// @lc code=end
