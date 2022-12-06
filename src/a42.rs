/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut monotonic_stack = vec![];
        let mut ans = 0;
        let mut max_idx = 0;
        monotonic_stack.push(0);
        for idx in 0..height.len() {
            let last = *monotonic_stack.last().unwrap();
            if height[idx] >= height[last] {
                max_idx = idx;
                monotonic_stack.push(idx);
                ans += (idx - last) as i32 * height[last];
                for i in last..idx {
                    ans -= height[i];
                }
            }
        }

        let mut height = height;
        height.reverse();
        monotonic_stack.clear();
        monotonic_stack.push(0);
        for idx in 0..height.len() - max_idx {
            let last = *monotonic_stack.last().unwrap();
            if height[idx] > height[last] {
                monotonic_stack.push(idx);
                ans += (idx - last) as i32 * height[last];
                for i in last..idx {
                    ans -= height[i];
                }
            }
        }
        ans
    }
}
// @lc code=end
