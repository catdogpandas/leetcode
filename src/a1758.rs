/*
 * @lc app=leetcode.cn id=1758 lang=rust
 *
 * [1758] 生成交替二进制字符串的最少操作数
 */
struct Solution;
// @lc code=start
use std::mem::swap;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut a = '0';
        let mut b = '1';
        let res = s.chars().fold((0, 0), |res, c| {
            let res = match (a == c, b == c) {
                (true, true) => res,
                (true, false) => (res.0, res.1 + 1),
                (false, true) => (res.0 + 1, res.1),
                (false, false) => (res.0 + 1, res.1 + 1),
            };
            swap(&mut a, &mut b);
            res
        });
        res.0.min(res.1)
    }
}
// @lc code=end
