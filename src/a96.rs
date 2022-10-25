/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut ans = 1 as i64;
        for i in 0..n as usize {
            ans = ans * 2 * (2 * i as i64 + 1) / (i as i64 + 2);
        }
        ans as i32
    }
}
// @lc code=end
