/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn dfs(s: &str, p: &str) -> bool {
            if s.len() == 0 && p.len() == 0 {
                return true;
            }
            if s.len() > 0 && p.len() == 0 {
                return false;
            }
            if p.len() == 1 || p.chars().nth(1) != Some('*') {
                if s.chars().nth(0) == p.chars().nth(0)
                    || s.chars().nth(0).is_some() && p.chars().nth(0) == Some('.')
                {
                    return dfs(&s[1..], &p[1..]);
                }
                return false;
            }
            let p_val = p.chars().nth(0).unwrap();
            if p_val == '.' && p.chars().nth(1) == Some('*') {
                let mut i = 0;
                if dfs(&s[i..], &p[2..]) {
                    return true;
                }
                while i < s.len() {
                    i += 1;
                    if dfs(&s[i..], &p[2..]) {
                        return true;
                    }
                }
            } else if p.chars().nth(1) == Some('*') {
                let mut i = 0;
                if dfs(&s[i..], &p[2..]) {
                    return true;
                }
                while i < s.len() {
                    if s.chars().nth(i) != Some(p_val) {
                        break;
                    }
                    i += 1;
                    if dfs(&s[i..], &p[2..]) {
                        return true;
                    }
                }
            }

            false
        }

        dfs(&s[..], &p[..])
    }
}
// @lc code=end
