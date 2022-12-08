/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
struct Solution {}
// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut ret = 0;
        let s_arr: Vec<char> = s.chars().collect();
        let mut cache = HashSet::new();

        s_arr.iter().enumerate().for_each(|(i, ch)| {
            while cache.contains(ch) {
                cache.remove(&s_arr[l as usize]);
                l += 1;
            }
            cache.insert(ch);
            ret = ret.max(r - l + 1);
            r += 1;
        });
        ret
    }
}
// @lc code=end
