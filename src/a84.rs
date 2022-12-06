/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;
        stack.push(0);
        let mut heights = heights;
        heights.insert(0, 0);
        heights.push(0);
        for idx in 1..heights.len() {
            let last = *stack.last().unwrap();
            if heights[idx] >= heights[last] {
                stack.push(idx);
            } else {
                while !stack.is_empty() {
                    let last = *stack.last().unwrap();
                    if heights[idx] >= heights[last] {
                        stack.push(idx);
                        break;
                    }
                    let left = stack[stack.len() - 2];
                    ans = ans.max(heights[last] as i32 * (idx - left - 1) as i32);
                    stack.pop();
                }
                if stack.is_empty() {
                    stack.push(idx);
                }
            }
        }

        ans
    }
}
// @lc code=end
