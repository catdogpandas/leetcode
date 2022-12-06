/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 * [279] 完全平方数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let is_perfect_square = |x| {
            let y = (x as f64).sqrt() as i32;
            return y * y == x;
        };
        let check_answar4 = |x| {
            let mut x = x;
            while x % 4 == 0 {
                x /= 4;
            }
            return x % 8 == 7;
        };
        if is_perfect_square(n) {
            return 1;
        }
        if check_answar4(n) {
            return 4;
        }
        let mut i = 1;
        while i * i <= n {
            if is_perfect_square(n - i * i) {
                return 2;
            }
            i += 1;
        }
        return 3;
    }
}
// @lc code=end
