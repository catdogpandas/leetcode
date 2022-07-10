/*
 * @lc app=leetcode.cn id=542 lang=rust
 *
 * [542] 01 矩阵
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn bfs(mat: &mut Vec<Vec<i32>>, i: usize, j: usize) {
            if i + 1 < mat.len() && mat[i][j] + 1 < mat[i + 1][j] {
                mat[i + 1][j] = mat[i][j] + 1;
                bfs(mat, i + 1, j);
            }
            if i > 0 && mat[i][j] + 1 < mat[i - 1][j] {
                mat[i - 1][j] = mat[i][j] + 1;
                bfs(mat, i - 1, j);
            }
            if j + 1 < mat[0].len() && mat[i][j] + 1 < mat[i][j + 1] {
                mat[i][j + 1] = mat[i][j] + 1;
                bfs(mat, i, j + 1);
            }
            if j > 0 && mat[i][j] + 1 < mat[i][j - 1] {
                mat[i][j - 1] = mat[i][j] + 1;
                bfs(mat, i, j - 1);
            }
        }
        let mut ans = mat;
        for i in 0..ans.len() {
            for j in 0..ans[0].len() {
                if ans[i][j] != 0 {
                    ans[i][j]=10000;
                }
            }
        }
        for i in 0..ans.len() {
            for j in 0..ans[0].len() {
                if ans[i][j] == 0 {
                    bfs(&mut ans, i , j);
                }
            }
        }
        ans
    }
}
// @lc code=end
