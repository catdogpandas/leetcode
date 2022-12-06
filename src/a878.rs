/*
 * @lc app=leetcode.cn id=878 lang=rust
 *
 * [878] 第 N 个神奇数字
 */
struct Solution;
// @lc code=start
use std::cmp;
const MOD: i64 = 1000000007;
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if a % b == 0 {
                b
            } else {
                gcd(b, a % b)
            }
        }
        let c = gcd(a, b);
        let lcm = a * b / c;
        let cnt = lcm / a + lcm / b - 1;
        let (d, r) = (n / cnt, n % cnt);
        let mut ans: i64 = d as i64 * lcm as i64 % MOD;
        if r != 0 {
            let (mut amul, mut bmul) = (a, b);
            for _ in 0..r as usize {
                if amul == bmul {
                    amul += a;
                    bmul += b;
                } else if amul < bmul {
                    amul += a;
                } else {
                    bmul += b;
                }
            }
            ans = (ans + cmp::max(amul - a, bmul - b) as i64) % MOD;
        }

        ans as i32
    }
}
// @lc code=end
