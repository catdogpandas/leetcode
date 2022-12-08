/*
 * @lc app=leetcode.cn id=994 lang=rust
 *
 * [994] 腐烂的橘子
 */
struct Solution {}
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut q = VecDeque::new();
        let mut ans = 0;
        let mut cur_len = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    q.push_back((i, j));
                }
            }
        }

        cur_len = q.len();
        if cur_len == 0 {
            ans += 1;
        }
        while !q.is_empty() {
            if let Some((i, j)) = q.pop_front() {
                if i > 0 && grid[i - 1][j] == 1 {
                    grid[i - 1][j] = 2;
                    q.push_back((i - 1, j));
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    grid[i][j - 1] = 2;
                    q.push_back((i, j - 1));
                }
                if i + 1 < grid.len() && grid[i + 1][j] == 1 {
                    grid[i + 1][j] = 2;
                    q.push_back((i + 1, j));
                }
                if j + 1 < grid[0].len() && grid[i][j + 1] == 1 {
                    grid[i][j + 1] = 2;
                    q.push_back((i, j + 1));
                }
                cur_len -= 1;
                if cur_len == 0 {
                    cur_len = q.len();
                    ans += 1;
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    return -1;
                }
            }
        }
        ans - 1
    }
}
// @lc code=end
