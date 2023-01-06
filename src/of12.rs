/*
 * @lc app=leetcode.cn id=剑指 Offer 12 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 12] 矩阵中的路径
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &Vec<Vec<char>>,
            visit: &mut Vec<Vec<bool>>,
            word: &Vec<char>,
            idx: usize,
            i: usize,
            j: usize,
        ) -> bool {
            if idx >= word.len() {
                return true;
            }
            if i >= board.len() || j >= board[0].len() {
                return false;
            }
            if visit[i][j] {
                return false;
            }
            if board[i][j] == word[idx] {
                visit[i][j] = true;
                if dfs(board, visit, word, idx + 1, i + 1, j)
                    || dfs(board, visit, word, idx + 1, i - 1, j)
                    || dfs(board, visit, word, idx + 1, i, j + 1)
                    || dfs(board, visit, word, idx + 1, i, j - 1)
                {
                    return true;
                }
                visit[i][j] = false;
            }
            false
        }
        let mut visit = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if dfs(&board, &mut visit, &word.chars().collect(), 0, i, j) {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end

/*
// @lcpr case=start
// [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]\n"ABCCED"\n
// @lcpr case=end

// @lcpr case=start
// [["a","b"],["c","d"]]\n"abcd"\n
// @lcpr case=end

 */
