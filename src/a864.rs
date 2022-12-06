/*
 * @lc app=leetcode.cn id=864 lang=rust
 *
 * [864] 获取所有钥匙的最短路径
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let mut keys = 0;
        let grid = grid
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut i_start = 0;
        let mut j_start = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                match grid[i][j] {
                    '@' => {
                        i_start = i;
                        j_start = j;
                    }
                    c if c >= 'a' && c <= 'f' => {
                        keys ^= 1 << (c as u8 - 'a' as u8) as i32;
                    }
                    _ => continue,
                }
            }
        }
        let mut visit = vec![vec![vec![-1; 128]; grid[0].len()]; grid.len()];
        let mut queue = VecDeque::new();
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        visit[i_start][j_start][keys] = 0;
        queue.push_back((i_start, j_start, keys));
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            for dir in &dirs {
                let nx = (cur.0 as i32 + dir.0) as usize;
                let ny = (cur.1 as i32 + dir.1) as usize;
                let x = cur.0;
                let y = cur.1;
                let keys = cur.2;
                if nx < grid.len() && ny < grid[nx].len() && grid[nx][ny] != '#' {
                    match grid[nx][ny] {
                        '.' | '@' => {
                            if visit[nx][ny][keys] == -1 {
                                visit[nx][ny][keys] = visit[x][y][keys] + 1;
                                queue.push_back((nx, ny, keys));
                            }
                        }
                        c if c >= 'a' && c <= 'f' => {
                            if visit[nx][ny][keys & !(1 << (c as u8 - 'a' as u8))] == -1 {
                                visit[nx][ny][keys & !(1 << (c as u8 - 'a' as u8))] =
                                    visit[x][y][keys] + 1;
                                if keys & !(1 << (c as u8 - 'a' as u8)) == 0 {
                                    return visit[nx][ny][keys & !(1 << (c as u8 - 'a' as u8))];
                                }
                                queue.push_back((nx, ny, keys & !(1 << (c as u8 - 'a' as u8))));
                            }
                        }
                        c => {
                            if keys & (1 << (c as u8 - 'A' as u8)) == 0 && visit[nx][ny][keys] == -1
                            {
                                visit[nx][ny][keys] = visit[x][y][keys] + 1;
                                queue.push_back((nx, ny, keys));
                            }
                        }
                    }
                }
            }
        }
        -1
    }
}
// @lc code=end
