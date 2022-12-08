/*
 * @lc app=leetcode.cn id=567 lang=rust
 *
 * [567] 字符串的排列
 */
struct Solution {}
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.into_bytes(), s2.into_bytes());
        if s2.len() < s1.len() {
            return false;
        }
        let (mut map1, mut map2) = (
            HashMap::with_capacity(s1.len()),
            HashMap::with_capacity(s1.len()),
        );
        s1.iter().for_each(|c| {
            *map1.entry(c).or_insert(0) += 1;
        });
        s2.iter().take(s1.len()).for_each(|c| {
            *map2.entry(c).or_insert(0) += 1;
        });
        if map1 == map2 {
            return true;
        }
        for i in 1..s2.len() - s1.len() + 1 {
            if let Some(v) = map2.get(&s2[i - 1]) {
                if *v <= 1 {
                    map2.remove(&s2[i - 1]);
                } else {
                    *map2.entry(&s2[i - 1]).or_insert(0) -= 1;
                }
            }
            *map2.entry(&s2[s1.len() - 1 + i]).or_insert(0) += 1;
            if map1 == map2 {
                return true;
            }
        }
        false
    }
}
// @lc code=end
