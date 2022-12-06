/*
 * @lc app=leetcode.cn id=1106 lang=rust
 *
 * [1106] 解析布尔表达式
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let expression: Vec<_> = expression.chars().rev().collect();
        let mut stack = vec![];
        for c in expression {
            match c {
                '&' => {
                    let mut cur = true;
                    while let Some(x) = stack.pop() {
                        match x {
                            ')' => {
                                stack.push(if cur { 't' } else { 'f' });
                                break;
                            }
                            'f' => cur = false,
                            _ => continue,
                        }
                    }
                }
                '|' => {
                    let mut cur = false;
                    while let Some(x) = stack.pop() {
                        match x {
                            ')' => {
                                stack.push(if cur { 't' } else { 'f' });
                                break;
                            }
                            't' => cur = true,
                            _ => continue,
                        }
                    }
                }
                '!' => {
                    let mut cur = true;
                    while let Some(x) = stack.pop() {
                        match x {
                            ')' => {
                                stack.push(if cur { 'f' } else { 't' });
                                break;
                            }
                            't' => cur = true,
                            'f' => cur = false,
                            _ => continue,
                        }
                    }
                }
                '(' => continue,
                ',' => continue,
                _ => stack.push(c),
            }
        }
        let ans = stack.pop().unwrap();
        if ans == 't' {
            true
        } else {
            false
        }
    }
}
// @lc code=end
