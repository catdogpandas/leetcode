/*
 * @lc app=leetcode.cn id=882 lang=rust
 *
 * [882] 细分图中的可到达结点
 */
struct Solution;
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];

        for edge in &edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2] as usize));
            graph[edge[1] as usize].push((edge[0] as usize, edge[2] as usize));
        }

        let mut vec = vec![usize::MAX; n as usize];
        let mut bh = BinaryHeap::new();
        vec[0] = 0;
        bh.push(Reverse((0, 0)));

        while let Some(Reverse((x, y))) = bh.pop() {
            if x != vec[y] {
                continue;
            }

            for &(u, v) in &graph[y] {
                let w = x + v + 1;

                if w < vec[u] {
                    vec[u] = w;
                    bh.push(Reverse((w, u)));
                }
            }
        }
 
        let max_moves = max_moves as usize;
        let mut ans = vec.iter().filter(|&&v| v <= max_moves).count();

        for (i, j, v) in edges
            .into_iter()
            .map(|edge| (edge[0] as usize, edge[1] as usize, edge[2] as usize))
        {
            ans += v.min(max_moves.saturating_sub(vec[i]) + max_moves.saturating_sub(vec[j]));
        }

        ans as i32
    }
}
// @lc code=end
