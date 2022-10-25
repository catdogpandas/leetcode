/*
 * @lc app=leetcode.cn id=902 lang=rust
 *
 * [902] 最大为 N 的数字组合
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let lens = digits.len() as i32;
        let n_str = n.to_string();
        let n_str = n_str.as_bytes();
        let (lsum, esum) = n_str
            .iter()
            .enumerate()
            .fold((0, 1), |(lsum, esum), (idx, c)| {
                let mut flag = 0;
                let lcnt = digits.iter().fold(0, |cnt, s| {
                    if s.as_bytes().first().unwrap() < c {
                        return cnt + 1;
                    }
                    if s.as_bytes().first().unwrap() == c {
                        flag = 1;
                    }
                    cnt
                });
                if idx == 0 {
                    return (lcnt * esum, (flag & esum));
                }
                return (lens + lsum * lens + lcnt * esum, (flag & esum));
            });
        lsum + esum
    }
}
// @lc code=end
