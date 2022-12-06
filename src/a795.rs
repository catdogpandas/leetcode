/*
 * @lc app=leetcode.cn id=795 lang=rust
 *
 * [795] 区间子数组个数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        let (mut last1, mut last2) = (-1, -1);
        for i in 0..nums.len() {
            match nums[i] {
                x if x < left => {}
                x if x > right => {
                    last2 = i as i32;
                    last1 = -1;
                }
                _ => {
                    last1 = i as i32;
                }
            }
            if last1 != -1 {
                ans += last1 - last2;
            }
        }
        ans as i32
    }
}
// @lc code=end
