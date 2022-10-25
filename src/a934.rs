/*
 * @lc app=leetcode.cn id=934 lang=rust
 *
 * [934] 最短的桥
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut dist = vec![vec![i32::MAX; grid.len()]; grid.len()];

        'exit: for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 1 {
                    Solution::dfs(&mut grid, i, j, &mut queue, &mut dist);
                    break 'exit;
                }
            }
        }

        Solution::bfs(grid, queue, dist)
    }

    fn bfs(
        grid: Vec<Vec<i32>>,
        mut queue: VecDeque<(usize, usize)>,
        mut dist: Vec<Vec<i32>>,
    ) -> i32 {
        let direction = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (x, y) = queue.pop_front().unwrap();
                if grid[x][y] == 1 {
                    return dist[x][y] - 1;
                }
                for item in &direction {
                    let new_x = (x as i32 + item[0]) as usize;
                    let new_y = (y as i32 + item[1]) as usize;
                    if new_x < grid.len() && new_y < grid.len() {
                        if dist[x][y] + 1 < dist[new_x][new_y] {
                            dist[new_x][new_y] = dist[x][y] + 1;
                            queue.push_back((new_x, new_y));
                        }
                    }
                }
            }
        }
        -1
    }

    fn dfs(
        grid: &mut [Vec<i32>],
        x: usize,
        y: usize,
        queue: &mut VecDeque<(usize, usize)>,
        dist: &mut [Vec<i32>],
    ) {
        if x >= grid.len() || y >= grid.len() {
            return;
        }
        if grid[x][y] != 1 {
            return;
        }
        grid[x][y] = 0;
        queue.push_back((x, y));
        dist[x][y] = 0;

        Solution::dfs(grid, x - 1, y, queue, dist);
        Solution::dfs(grid, x, y + 1, queue, dist);
        Solution::dfs(grid, x + 1, y, queue, dist);
        Solution::dfs(grid, x, y - 1, queue, dist);
    }
}
// @lc code=end
