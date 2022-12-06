/*
 * @lc app=leetcode.cn id=85 lang=rust
 *
 * [85] 最大矩形
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
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
        let dp = matrix
            .into_iter()
            .map(|row| {
                let mut tmp = 0;
                row.into_iter()
                    .map(|c| {
                        if c == '0' {
                            tmp = 0;
                        } else {
                            tmp += 1;
                        }
                        tmp
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();
        let mut ans = 0;
        for i in 0..dp[0].len() {
            ans = ans.max(largest_rectangle_area(
                dp.iter().map(|row| row[i]).collect(),
            ));
        }
        ans
    }
}
// @lc code=end
