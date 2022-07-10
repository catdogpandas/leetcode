/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */
struct Solution {}
// @lc code=start
pub fn dfs(n: i32, k: i32, idx: i32, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if cur.len() == k as usize {
        result.push(cur.to_vec());
        return;
    }
    for i in idx..=n {
        cur.push(i);
        dfs(n, k, i + 1, cur, result);
        cur.pop();
    }
}
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // 递归
        let mut cur: Vec<i32> = vec![];
        let mut result: Vec<Vec<i32>> = vec![];
        dfs(n, k, 1, &mut cur, &mut result);
        result
    }
}
// @lc code=end
