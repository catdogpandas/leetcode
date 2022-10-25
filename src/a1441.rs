/*
 * @lc app=leetcode.cn id=1441 lang=rust
 *
 * [1441] 用栈操作构建数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = vec![];
        let push = "Push";
        let pop = "Pop";
        let mut cur = 1;
        let mut stack = vec![];
        for item in target {
            stack.push(cur);
            ans.push(push.to_string());
            cur += 1;
            while let Some(&x) = stack.last() {
                if x == item {
                    break;
                } else {
                    stack.pop();
                    ans.push(pop.to_string());
                    stack.push(cur);
                    cur += 1;
                    ans.push(push.to_string());
                }
            }
        }
        ans
    }
}
// @lc code=end
