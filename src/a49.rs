/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 */
struct Solution {}
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let prime: Vec<u128> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        strs.iter()
            .fold(HashMap::<_, Vec<String>>::new(), |mut map, s| {
                map.entry(
                    s.chars()
                        .fold(1, |p, c| p * prime[c as usize - 'a' as usize]),
                )
                .and_modify(|v| v.push(s.to_string()))
                .or_insert_with(|| vec![s.to_string()]);
                map
            })
            .drain()
            .map(|(_, v)| v)
            .collect()
    }
}
// @lc code=end
