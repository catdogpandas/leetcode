/*
 * @lc app=leetcode.cn id=1668 lang=rust
 *
 * [1668] 最大重复子字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        // kmp
        let s = word.as_bytes();
        let mut pi = vec![0; word.len()];
        for i in 1..word.len() {
            let mut j = pi[i - 1];
            while j > 0 && s[i] != s[j] {
                j = pi[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            pi[i] = j;
        }
        let seq = sequence.as_bytes();
        let mut f = vec![0; sequence.len()];
        let mut j = 0;
        for i in 0..sequence.len() {
            while j != 0 && s[j] != seq[i] {
                j = pi[j - 1];
            }
            if s[j] == seq[i] {
                j += 1;
                if j == word.len() {
                    if i >= word.len() {
                        f[i] = f[i - word.len()] + 1;
                    } else {
                        f[i] = 1;
                    }
                    j = pi[j - 1];
                }
            }
        }
        *f.iter().max().unwrap()
    }
}
// @lc code=end
