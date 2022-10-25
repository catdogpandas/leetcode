/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            if i == 0 {
                stack.push(i);
            } else {
                let mut front = stack.last().unwrap();
                if temperatures[*front] < temperatures[i] {
                    while temperatures[*front] < temperatures[i] {
                        ans[*front] = (i - front) as i32;
                        stack.pop();
                        if stack.is_empty() {
                            break;
                        }
                        front = stack.last().unwrap();
                    }
                }
                stack.push(i);
            }
        }
        ans
    }
}
// @lc code=end
