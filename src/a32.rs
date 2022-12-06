/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;
        stack.push(-2);
        for c in s.chars() {
            if c == '(' {
                stack.push(-1);
            } else {
                let mut add = 0;
                while !stack.is_empty() {
                    let tmp = *stack.last().unwrap();
                    if tmp == -1 {
                        stack.pop();
                        stack.push(2 + add);
                        break;
                    } else if tmp == -2 {
                        if add > 0 {
                            stack.push(add);
                        }
                        stack.push(-2);
                        break;
                    } else {
                        add += tmp;
                        stack.pop();
                    }
                }
            }
        }
        let mut cnt = 0;
        for item in stack {
            if item > 0 {
                cnt += item;
                ans = ans.max(cnt);
            } else {
                cnt = 0;
            }
        }
        ans
    }
}
// @lc code=end
