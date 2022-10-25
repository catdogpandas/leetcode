/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for circle in 0..(len / 2) {
            for idx in 0..(len - 1 - circle - circle) {
                let tmp = matrix[circle][circle + idx];
                matrix[circle][circle + idx] = matrix[len - 1 - circle - idx][circle];
                matrix[len - 1 - circle - idx][circle] =
                    matrix[len - 1 - circle][len - 1 - circle - idx];
                matrix[len - 1 - circle][len - 1 - circle - idx] =
                    matrix[circle + idx][len - 1 - circle];
                matrix[circle + idx][len - 1 - circle] = tmp;
            }
        }
    }
}
// @lc code=end
