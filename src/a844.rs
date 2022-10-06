/*
 * @lc app=leetcode.cn id=844 lang=rust
 *
 * [844] 比较含退格的字符串
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_right = s.len() as i32 - 1;
        let mut t_right = t.len() as i32 - 1;
        let mut ans = true;
        while s_right >= 0 || t_right >= 0 {
            if s_right >= 0 && t_right >= 0 && s[s_right as usize] == t[t_right as usize] && s[s_right as usize]!=b'#'&& t[t_right as usize]!=b'#' {
                s_right -= 1;
                t_right -= 1;
            } else {
                if s_right >= 0 && s[s_right as usize] == b'#' {
                    let mut count = 1;
                    s_right -= 1;
                    while s_right >= 0 && count > 0 {
                        if s[s_right as usize] == b'#' {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                        s_right -= 1;
                    }
                } else if t_right >= 0 && t[t_right as usize] == b'#' {
                    let mut count = 1;
                    t_right -= 1;
                    while t_right >= 0 && count > 0 {
                        if t[t_right as usize] == b'#' {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                        t_right -= 1;
                    }
                }else{
                    return false
                }
            }
        }
        if s_right==t_right{
            return true;
        }
        false
    }
}
// @lc code=end
