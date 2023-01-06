/*
 * @lc app=leetcode.cn id=1697 lang=rust
 * @lcpr version=21006
 *
 * [1697] 检查边长度限制的路径是否存在
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        mut edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        edge_list.sort_unstable_by_key(|v| v[2]);
        let mut queries = queries
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, Vec<i32>)>>();
        queries.sort_unstable_by_key(|t| t.1[2]);

        let mut fa = (0..n as usize).collect::<Vec<usize>>();
        let mut ans = vec![false; queries.len()];
        let mut k = 0;
        queries.into_iter().for_each(|(i, v)| {
            while k < edge_list.len() && edge_list[k][2] < v[2] {
                Self::merge(&mut fa, edge_list[k][0] as usize, edge_list[k][1] as usize);
                k += 1;
            }
            ans[i] = (Self::find(&mut fa, v[0] as usize) == Self::find(&mut fa, v[1] as usize));
        });
        return ans;
    }

    fn find(fa: &mut Vec<usize>, target: usize) -> usize {
        if fa[target] != target {
            fa[target] = Self::find(fa, fa[target]);
        }
        return fa[target];
    }

    fn merge(fa: &mut Vec<usize>, from: usize, to: usize) {
        let idx = Self::find(fa, from);
        fa[idx] = Self::find(fa, to);
    }
}
// @lc code=end

/*
// @lcpr case=start
// 3\n[[0,1,2],[1,2,4],[2,0,8],[1,0,16]]\n[[0,1,2],[0,2,5]]\n
// @lcpr case=end

// @lcpr case=start
// 5\n[[0,1,10],[1,2,5],[2,3,9],[3,4,13]]\n[[0,4,14],[1,4,13]]\n
// @lcpr case=end

 */
