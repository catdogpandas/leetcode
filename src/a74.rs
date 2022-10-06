/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row_top = 0;
        let mut row_bottom = matrix.len() - 1;
        while row_top <= row_bottom {
            let mid = (row_bottom + row_top) / 2;
            if target >= matrix[mid][0] && target <= *matrix[mid].last().unwrap() {
                //binary search
                let mut left = 0;
                let mut right = matrix[mid].len() - 1;
                while left <= right {
                    let mid1 = (left + right) / 2;
                    if matrix[mid][mid1] == target {
                        return true;
                    } else if matrix[mid][mid1] < target {
                        left = mid1 + 1;
                    } else {
                        right = mid1 - 1;
                    }
                }
                return false;
            } else if target < matrix[mid][0] {
                if mid == 0 {
                    return false;
                }
                row_bottom = mid - 1;
            } else {
                row_top = mid + 1;
            }
        }
        false
    }
}
// @lc code=end
