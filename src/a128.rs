/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums_set = HashSet::new();
        for item in nums {
            nums_set.insert(item);
        }
        let mut ans = 0;
        for &item in nums_set.iter() {
            if !nums_set.contains(&(item - 1)) {
                let mut tmp = 1;
                let mut i = item + 1;
                while nums_set.contains(&i) {
                    tmp += 1;
                    i += 1;
                }
                ans = ans.max(tmp);
            }
        }
        ans
    }
}
// @lc code=end
