/*
 * @lc app=leetcode.cn id=809 lang=rust
 *
 * [809] 情感丰富的文字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        fn sp(s: String) -> Vec<String> {
            let mut l = 0;
            let mut res = vec![];
            let sv = s.chars().collect::<Vec<_>>();
            for i in 0..s.len() {
                if sv[i] == sv[l] {
                    continue;
                } else {
                    res.push(s[l..i].to_string());
                    l = i;
                }
            }
            res.push(s[l..].to_string());
            res
        }
        let t = sp(s);
        words.iter().map(|s| sp(s.to_string())).fold(0, |num, cur| {
            if t.len() == cur.len() {
                num + t.iter().zip(cur.iter()).fold(1, |res, (l, r)| {
                    if l.as_bytes()[0] == r.as_bytes()[0]
                        && (l.len() == r.len() || l.len() > r.len() && l.len() > 2)
                    {
                        res
                    } else {
                        0
                    }
                })
            } else {
                num
            }
        })
    }
}
// @lc code=end
