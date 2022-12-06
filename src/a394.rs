use std::ops::Add;

/*
 * @lc app=leetcode.cn id=394 lang=rust
 *
 * [394] 字符串解码
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn decode_string(s: String) -> String {
        if let Some((num_string, res)) = s.split_once("[") {
            let mut cnt = 0;
            for i in num_string.chars() {
                if i >= '0' && i <= '9' {
                    break;
                }
                cnt += 1;
            }
            let (pre, num_string) = num_string.split_at(cnt);

            let num = num_string.parse::<i32>().unwrap();
            cnt = 1;
            let mut p = 0;
            let mut it = res.chars();
            while cnt != 0 {
                let tmp = it.next().unwrap();
                p += 1;
                if '[' == tmp {
                    cnt += 1;
                } else if tmp == ']' {
                    cnt -= 1;
                }
            }
            let (mid, res) = res.split_at(p);
            let mid_string = Self::decode_string(mid[..mid.len() - 1].to_string());
            let res_string = Self::decode_string(res.to_string());
            return pre.to_string() + &mid_string.repeat(num as usize) + &res_string;
        }

        s
    }
}
// @lc code=end
