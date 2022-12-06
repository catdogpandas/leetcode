/*
 * @lc app=leetcode.cn id=剑指 Offer 04 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 04] 二维数组中的查找
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }
        let mut m = 0;
        let mut n = matrix[0].len() - 1;
        while m < matrix.len() && n < matrix[0].len() {
            if matrix[m][n] == target {
                return true;
            } else if matrix[m][n] < target {
                m += 1;
            } else {
                n -= 1;
            }
        }
        false
    }
}
// @lc code=end
