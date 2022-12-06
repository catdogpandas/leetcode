/*
 * @lc app=leetcode.cn id=301 lang=rust
 *
 * [301] 删除无效的括号
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn calc_min_need(s: &str) -> i32 {
            let mut cnt = 0;
            s.chars().fold(0, |res, c| {
                match c {
                    '(' => {
                        cnt += 1;
                        return res;
                    }
                    ')' => {
                        if cnt > 0 {
                            cnt -= 1;
                            return res;
                        } else {
                            return res + 1;
                        }
                    }
                    _ => {}
                }
                res
            }) + cnt
        }
        fn dfs(s: String, ans: &mut HashSet<String>, min_needs: i32) -> bool {
            if min_needs == 0 {
                if calc_min_need(&s[..]) == 0 {
                    ans.insert(s);
                    return true;
                }
                return false;
            }
            //dfs
            let len = s.len();
            let mut cnt = 0;
            for i in 0..len {
                let mut s_clone = s.clone();
                match s.chars().nth(i) {
                    Some('(') => {
                        cnt += 1;
                        if s.chars().nth(i) == s.chars().nth(i - 1) {
                            continue;
                        }
                        s_clone.remove(i);
                        dfs(s_clone, ans, min_needs - 1);
                    }
                    Some(')') => {
                        cnt -= 1;
                        if s.chars().nth(i) == s.chars().nth(i - 1) {
                            continue;
                        }
                        s_clone.remove(i);
                        dfs(s_clone, ans, min_needs - 1);
                    }
                    _ => {}
                }
                if cnt < 0 {
                    break;
                }
            }
            false
        }
        use std::collections::HashSet;
        let mut ans = HashSet::new();

        let min_needs = calc_min_need(&s[..]);
        dfs(s, &mut ans, min_needs);
        ans.into_iter().collect()
    }
}
// @lc code=end
