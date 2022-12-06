/*
 * @lc app=leetcode.cn id=754 lang=rust
 *
 * [754] 到达终点数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        let n = (((target as i64 * 8 - 1) as f64).sqrt() / 2.0 - 0.5) as i32;
        let d = target - n * (n + 1) / 2;
        if d == 0 {
            return n;
        }
        let d = n + 1 - d;
        if d % 2 == 0 {
            return n + 1;
        }
        if (d + n) % 2 == 0 {
            return n + 2;
        }
        n + 3
    }
}
// @lc code=end
