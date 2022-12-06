use std::borrow::Borrow;

/*
 * @lc app=leetcode.cn id=792 lang=rust
 *
 * [792] 匹配子序列的单词数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort();
        let mut pos = vec![vec![]; 26];
        let s_bytes = s.bytes().collect::<Vec<_>>();
        for i in 0..s_bytes.len() {
            pos[(s_bytes[i] - 'a' as u8) as usize].push(i);
        }
        let mut res = 0;
        for item in words {
            let mut cur = 0;
            for c in item.bytes() {
                let c_pos = &pos[(c - 'a' as u8) as usize];
                let p = c_pos.binary_search(&cur);
                if p.is_ok() {
                    let mut p = p.unwrap();
                    while p < c_pos.len() && p > 0 && p > cur && c_pos[p] == c_pos[p - 1] {
                        p = p - 1;
                    }
                    cur = p;
                } else {
                    break;
                }
            }
        }
        0
    }
}
// @lc code=end
