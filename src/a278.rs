/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 */
struct Solution {}
// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        let mut ans = n;
        while left <= right {
            let mid = left + (-left + right) / 2;
            if self.isBadVersion(mid) {
                ans = ans.min(mid);
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans
    }
}
// @lc code=end
