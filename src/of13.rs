/*
 * @lc app=leetcode.cn id=面试题13 lang=rust
 * @lcpr version=21104
 *
 * [面试题13] 机器人的运动范围
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        fn dfs(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            si: i32,
            sj: i32,
            k: i32,
            visited: &mut Vec<Vec<bool>>,
        ) -> i32 {
            if i >= m || j >= n || visited[i][j] || si + sj > k {
                return 0;
            }
            visited[i][j] = true;
            1 + dfs(
                i + 1,
                j,
                m,
                n,
                si + if (i + 1) % 10 == 0 { -8 } else { 1 },
                sj,
                k,
                visited,
            ) + dfs(
                i,
                j + 1,
                m,
                n,
                si,
                sj + if (j + 1) % 10 == 0 { -8 } else { 1 },
                k,
                visited,
            )
        }
        let mut visited = vec![vec![false; n as usize]; m as usize];
        dfs(0, 0, m as usize, n as usize, 0, 0, k, &mut visited)
    }
}
// @lc code=end

/*
// @lcpr case=start
// 2\n3\n1\n
// @lcpr case=end

// @lcpr case=start
// 3\n1\n0\n
// @lcpr case=end

 */
