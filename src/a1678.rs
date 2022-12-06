/*
 * @lc app=leetcode.cn id=1678 lang=rust
 *
 * [1678] 设计 Goal 解析器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut command = command;
        while command.contains("(al)") {
            command = command.replace("(al)", "al");
        }
        command.replace("()", "o")
    }
}
// @lc code=end
