/*
 * @lc app=leetcode.cn id=790 lang=rust
 *
 * [790] 多米诺和托米诺平铺
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let mut a = vec![0; n + 3];
        let mut b = vec![0; n + 3];
        a[0] = 1;

        for i in 0..n {
            a[i + 1] += (a[i] + b[i]) % 1000000007;
            a[i + 1] %= 1000000007;

            a[i + 2] += a[i];
            a[i + 2] %= 1000000007;

            b[i + 1] += b[i];
            b[i + 1] %= 1000000007;

            b[i + 2] += (2 * a[i]) % 1000000007;
            b[i + 2] %= 1000000007;
        }
        a[n] % 1000000007
    }
}
// @lc code=end
