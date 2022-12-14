/*
 * @lc app=leetcode.cn id=461 lang=rust
 *
 * [461] 汉明距离
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
// @lc code=end
